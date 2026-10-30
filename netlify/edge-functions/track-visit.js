const DISCORD_WEBHOOK_URL_ENV = "DISCORD_WEBHOOK_URL";
const TRACKING_ENABLED_ENV = "VISIT_TRACKING_ENABLED";
const DISCORD_USERNAME_ENV = "DISCORD_WEBHOOK_USERNAME";
const DISCORD_AVATAR_URL_ENV = "DISCORD_WEBHOOK_AVATAR_URL";

const EMBED_COLOR = 0x4b0082;

export default async function handler(request, context) {
  const url = new URL(request.url);
  const accept = request.headers.get("accept") || "";
  const purpose = `${request.headers.get("purpose") || ""} ${request.headers.get("sec-purpose") || ""}`.toLowerCase();
  const isHomePageView =
    request.method === "GET" &&
    url.pathname === "/" &&
    accept.includes("text/html") &&
    !purpose.includes("prefetch") &&
    !purpose.includes("prerender") &&
    request.headers.get("x-moz") !== "prefetch";

  if (!isHomePageView) {
    return context.next();
  }

  context.waitUntil(
    trackVisit(request, context, url).catch((error) => {
      console.error("[track-visit] unexpected error:", error?.message ?? error);
    }),
  );

  return context.next();
}

export const config = {
  path: "/",
};

async function trackVisit(request, context, url) {
  const env = Netlify.env;

  if (env.get(TRACKING_ENABLED_ENV) === "false") {
    return;
  }

  const webhookUrl = env.get(DISCORD_WEBHOOK_URL_ENV);
  if (!webhookUrl) {
    console.warn(`[track-visit] ${DISCORD_WEBHOOK_URL_ENV} is not set; skipping`);
    return;
  }

  const now = new Date();
  const unix = Math.floor(now.getTime() / 1000);
  const userAgent = request.headers.get("user-agent") || "";
  const device = describeDevice(userAgent);
  const geo = context.geo ?? null;

  const fields = [
    {
      name: "IP",
      value: code(getClientIp(request, context) || "desconhecido"),
      inline: true,
    },
    {
      name: "Localização",
      value: code(formatLocation(geo)),
      inline: true,
    },
    {
      name: "Hora",
      value: `<t:${unix}:F>`,
      inline: true,
    },
    {
      name: "Dispositivo",
      value: code(device.label),
      inline: true,
    },
    {
      name: "Página",
      value: code(`${url.pathname}${url.search}`),
      inline: true,
    },
    {
      name: "Referência",
      value: code(request.headers.get("referer") || "acesso direto"),
      inline: true,
    },
    {
      name: "Idioma",
      value: code(request.headers.get("accept-language") || "desconhecido"),
      inline: true,
    },
    {
      name: "User-Agent",
      value: code(userAgent || "desconhecido"),
      inline: false,
    },
  ];

  const payload = {
    username: truncate(env.get(DISCORD_USERNAME_ENV) || "Tesseract • Visitas", 80),
    allowed_mentions: { parse: [] },
    embeds: [
      {
        title: "Novo acesso ao site",
        color: EMBED_COLOR,
        timestamp: now.toISOString(),
        fields,
        footer: {
          text: truncate(
            `${context.site?.name || "Tesseract"} • ${context.site?.url || url.origin}`,
            2048,
          ),
        },
      },
    ],
  };

  const avatarUrl = env.get(DISCORD_AVATAR_URL_ENV);
  if (avatarUrl) {
    payload.avatar_url = avatarUrl;
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 5000);

  try {
    const response = await fetch(webhookUrl, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        "user-agent": "netlify-edge-function/track-visit",
      },
      body: JSON.stringify(payload),
      signal: controller.signal,
    });

    if (!response.ok) {
      const text = await response.text().catch(() => "");
      console.error(
        `[track-visit] Discord responded ${response.status}: ${text.slice(0, 500)}`,
      );
    }
  } catch (error) {
    console.error("[track-visit] Discord request failed:", error?.message ?? error);
  } finally {
    clearTimeout(timeout);
  }
}

function getClientIp(request, context) {
  return (
    context.ip ||
    request.headers.get("x-nf-client-connection-ip") ||
    request.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ||
    request.headers.get("client-ip") ||
    null
  );
}

function formatLocation(geo) {
  if (!geo) {
    return "desconhecida";
  }

  const parts = [
    geo.city,
    geo.subdivision?.name,
    geo.country?.name,
  ].filter(Boolean);

  return parts.length ? parts.join(", ") : "desconhecida";
}

function describeDevice(userAgent) {
  const ua = userAgent.toLowerCase();
  const isBot =
    /bot|crawler|spider|crawling|slurp|bingpreview|facebookexternalhit|headless/.test(
      ua,
    );
  const isTablet = /ipad|tablet|playbook|silk|android(?!.*mobile)/.test(ua);
  const isMobile =
    /mobile|iphone|ipod|android.*mobile|windows phone|blackberry|opera mini|iemobile/.test(
      ua,
    );

  const type = isBot ? "Bot" : isTablet ? "Tablet" : isMobile ? "Celular" : "Desktop";

  let os = "Sistema desconhecido";
  if (/windows nt 10/.test(ua)) os = "Windows 10/11";
  else if (/windows nt 6\.3/.test(ua)) os = "Windows 8.1";
  else if (/windows/.test(ua)) os = "Windows";
  else if (/android/.test(ua)) os = "Android";
  else if (/iphone|ipad|ipod/.test(ua)) os = "iOS";
  else if (/mac os x|macintosh/.test(ua)) os = "macOS";
  else if (/cros/.test(ua)) os = "ChromeOS";
  else if (/linux/.test(ua)) os = "Linux";
  else if (/freebsd/.test(ua)) os = "FreeBSD";

  let browser = "Navegador desconhecido";
  if (/edg\//.test(ua)) browser = "Edge";
  else if (/opr\/|opera/.test(ua)) browser = "Opera";
  else if (/firefox\//.test(ua)) browser = "Firefox";
  else if (/chrome\/|crios\//.test(ua)) browser = "Chrome";
  else if (/safari\//.test(ua)) browser = "Safari";
  else if (/msie|trident/.test(ua)) browser = "Internet Explorer";

  return {
    label: `${type} · ${os} · ${browser}`,
  };
}

function code(value) {
  return `\`${truncate(String(value).replaceAll("`", "'"), 1022)}\``;
}

function truncate(value, max) {
  return value.length > max ? `${value.slice(0, max - 1)}…` : value;
}
