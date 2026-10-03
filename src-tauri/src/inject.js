(() => {
  if (!/(^|\.)instagram\.com$/.test(location.hostname)) return;

  const HOME = "/direct/inbox/";

  // Routes that always redirect back to the inbox.
  const BLOCKED_PATHS = [
    /^\/$/,
    /^\/(explore|reels?)(\/|$)/,
    /^\/(your_activity|accounts\/activity)(\/|$)/,
    /^\/[^/]+\/saved(\/|$)/,
  ];

  // Sidebar and menu entries to hide (matched on visible text or icon label).
  const LABELS = new Set([
    "notifications",
    "new post",
    "create",
    "threads",
    "meta ai",
    "also from meta",
    "your activity",
    "saved",
  ]);

  const ITEM = 'a, button, [role="link"], [role="button"], [role="menuitem"]';

  const CSS =
    [
      'a[href="/"]',
      'a[href^="/explore"]',
      'a[href^="/reels"]',
      'a[href^="/your_activity"]',
      'a[href^="/accounts/activity"]',
      'a[href*="/saved/"]',
      'a[href*="threads.net"]',
      'a[href*="threads.com"]',
    ].join(",") + "{display:none!important}";

  const guard = () => {
    if (BLOCKED_PATHS.some((re) => re.test(location.pathname))) {
      location.replace(HOME);
    }
  };

  // Instagram is a single-page app, so hook client-side navigation too.
  for (const method of ["pushState", "replaceState"]) {
    const original = history[method];
    history[method] = function (...args) {
      const result = original.apply(this, args);
      guard();
      return result;
    };
  }
  window.addEventListener("popstate", guard);
  guard();

  const hide = (el) => {
    const item = el.closest(ITEM);
    if (item) item.style.setProperty("display", "none", "important");
  };

  // Everything outside <main> (sidebar, More menu) is scanned; chats are not.
  const scan = () => {
    document.querySelectorAll("svg[aria-label]:not(main svg)").forEach((svg) => {
      const label = svg.getAttribute("aria-label").trim().toLowerCase();
      if (LABELS.has(label)) hide(svg);
    });
    document.querySelectorAll("span:not(main span)").forEach((el) => {
      if (
        el.childElementCount === 0 &&
        LABELS.has(el.textContent.trim().toLowerCase())
      ) {
        hide(el);
      }
    });
  };

  const start = () => {
    const style = document.createElement("style");
    style.textContent = CSS;
    (document.head || document.documentElement).appendChild(style);

    let queued = false;
    new MutationObserver(() => {
      if (queued) return;
      queued = true;
      requestAnimationFrame(() => {
        queued = false;
        scan();
      });
    }).observe(document.documentElement, { childList: true, subtree: true });
    scan();
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", start);
  } else {
    start();
  }
})();
