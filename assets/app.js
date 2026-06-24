/**
 * app.js - Core interactive functionality for md-viewer
 * Handles: theme toggle, TOC scroll spy, sidebar toggle, TOC panel toggle, search
 */

(function () {
  'use strict';

  /* ── Theme ─────────────────────────────────────────────────── */
  const THEME_KEY = 'mdv-theme';

  function getTheme() {
    return localStorage.getItem(THEME_KEY) || 'dark';
  }

  function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(THEME_KEY, theme);
    const btn = document.getElementById('theme-btn');
    if (btn) {
      btn.setAttribute('aria-label', theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode');
      btn.innerHTML = theme === 'dark' ? iconSun() : iconMoon();
    }
  }

  window.toggleTheme = function () {
    const current = getTheme();
    setTheme(current === 'dark' ? 'light' : 'dark');
  };

  /* ── Left Sidebar ────────────────────────────────────────────── */
  const SIDEBAR_KEY = 'mdv-sidebar';

  function getSidebarState() {
    return localStorage.getItem(SIDEBAR_KEY) !== 'collapsed';
  }

  function setSidebarState(open) {
    localStorage.setItem(SIDEBAR_KEY, open ? 'open' : 'collapsed');
    applySlidebarState(open);
  }

  function applySlidebarState(open) {
    const sidebar = document.getElementById('sidebar');
    const wrapper = document.getElementById('content-wrapper');
    const overlay = document.getElementById('sidebar-overlay');
    if (!sidebar) return;

    const isMobile = window.innerWidth <= 996;

    if (isMobile) {
      sidebar.classList.toggle('open', open);
      if (overlay) overlay.classList.toggle('visible', open);
    } else {
      sidebar.classList.toggle('collapsed', !open);
      if (wrapper) wrapper.classList.toggle('sidebar-collapsed', !open);
    }

    const btn = document.getElementById('sidebar-btn');
    if (btn) btn.setAttribute('aria-pressed', String(open));
  }

  window.toggleSidebar = function () {
    setSidebarState(!getSidebarState());
  };

  /* ── TOC Panel (right side) ──────────────────────────────────── */
  const TOC_KEY = 'mdv-toc';

  /**
   * Get the saved TOC visibility state.
   * Default: visible if viewport width >= 1280px, hidden otherwise.
   */
  function getTocState() {
    const saved = localStorage.getItem(TOC_KEY);
    if (saved !== null) return saved === 'visible';
    return window.innerWidth >= 1280;
  }

  /**
   * Apply TOC panel visibility and update the toggle button icon.
   *
   * @param {boolean} visible
   */
  function applyTocState(visible) {
    const panel = document.querySelector('.toc-panel');
    if (!panel) return;
    panel.classList.toggle('visible', visible);

    const btn = document.getElementById('toc-btn');
    if (btn) {
      btn.setAttribute('aria-pressed', String(visible));
      btn.setAttribute('aria-label', visible ? 'Hide table of contents' : 'Show table of contents');
      btn.title = visible ? 'Hide table of contents' : 'Show table of contents';
    }
  }

  function setTocState(visible) {
    localStorage.setItem(TOC_KEY, visible ? 'visible' : 'hidden');
    applyTocState(visible);
  }

  window.toggleToc = function () {
    const panel = document.querySelector('.toc-panel');
    if (!panel) return;
    const isVisible = panel.classList.contains('visible');
    setTocState(!isVisible);
  };

  /* ── TOC Scroll Spy ─────────────────────────────────────────── */
  function initTocSpy() {
    const tocLinks = document.querySelectorAll('.toc-link');
    if (!tocLinks.length) return;

    const headings = Array.from(
      document.querySelectorAll('.markdown-body h1, .markdown-body h2, .markdown-body h3, .markdown-body h4')
    );
    if (!headings.length) return;

    let ticking = false;

    function updateActive() {
      const scrollY = window.scrollY;
      const offset = 80;

      let activeIdx = 0;
      for (let i = 0; i < headings.length; i++) {
        if (headings[i].getBoundingClientRect().top + scrollY - offset <= scrollY) {
          activeIdx = i;
        }
      }

      tocLinks.forEach(l => l.classList.remove('active'));
      const activeLink = document.querySelector(`.toc-link[href="#${CSS.escape(headings[activeIdx].id)}"]`);
      if (activeLink) {
        activeLink.classList.add('active');
        const tocPanel = document.querySelector('.toc-panel');
        if (tocPanel && tocPanel.classList.contains('visible')) {
          const linkTop = activeLink.offsetTop;
          const panelScrollTop = tocPanel.scrollTop;
          const panelHeight = tocPanel.clientHeight;
          if (linkTop < panelScrollTop || linkTop > panelScrollTop + panelHeight - 40) {
            tocPanel.scrollTop = linkTop - panelHeight / 2;
          }
        }
      }
    }

    window.addEventListener('scroll', function () {
      if (!ticking) {
        requestAnimationFrame(function () { updateActive(); ticking = false; });
        ticking = true;
      }
    }, { passive: true });

    updateActive();
  }

  /* ── Search ─────────────────────────────────────────────────── */
  function initSearch() {
    const input = document.getElementById('search-input');
    const results = document.getElementById('search-results');
    if (!input || !results || typeof Fuse === 'undefined' || typeof SEARCH_INDEX === 'undefined') return;

    const fuse = new Fuse(SEARCH_INDEX, {
      keys: [
        { name: 'title', weight: 0.4 },
        { name: 'headings', weight: 0.3 },
        { name: 'content', weight: 0.3 }
      ],
      threshold: 0.35,
      maxPatternLength: 32,
      includeScore: true,
      minMatchCharLength: 2
    });

    function renderResults(items) {
      results.innerHTML = '';
      if (!items.length) {
        results.innerHTML = '<div class="search-no-results">No results found</div>';
        results.classList.add('visible');
        return;
      }
      items.slice(0, 10).forEach(function (item) {
        const el = document.createElement('div');
        el.className = 'search-result-item';
        const snippet = (item.item.content || '').slice(0, 100).replace(/\n/g, ' ');
        el.innerHTML = `<div class="search-result-title">${escHtml(item.item.title)}</div>
          <div class="search-result-snippet">${escHtml(snippet)}</div>`;
        el.addEventListener('click', function () {
          window.location.href = item.item.path;
        });
        results.appendChild(el);
      });
      results.classList.add('visible');
    }

    input.addEventListener('input', function () {
      const q = input.value.trim();
      if (!q) { results.classList.remove('visible'); return; }
      renderResults(fuse.search(q));
    });

    input.addEventListener('focus', function () {
      if (input.value.trim()) renderResults(fuse.search(input.value.trim()));
    });

    document.addEventListener('click', function (e) {
      if (!e.target.closest('.search-wrapper')) {
        results.classList.remove('visible');
      }
    });

    input.addEventListener('keydown', function (e) {
      if (e.key === 'Escape') { input.blur(); results.classList.remove('visible'); }
    });
  }

  /* ── Heading anchor links ────────────────────────────────────── */
  function addAnchorLinks() {
    document.querySelectorAll('.markdown-body h1, .markdown-body h2, .markdown-body h3, .markdown-body h4, .markdown-body h5, .markdown-body h6')
      .forEach(function (h) {
        if (!h.id) return;
        const a = document.createElement('a');
        a.className = 'anchor-link';
        a.href = '#' + h.id;
        a.innerHTML = '#';
        a.setAttribute('aria-hidden', 'true');
        h.appendChild(a);
      });
  }

  /* ── SVG Icons ───────────────────────────────────────────────── */
  function iconSun() {
    return '<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>';
  }

  function iconMoon() {
    return '<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>';
  }

  /* ── Helpers ─────────────────────────────────────────────────── */
  function escHtml(str) {
    return String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  /* ── Init ────────────────────────────────────────────────────── */
  document.addEventListener('DOMContentLoaded', function () {
    // Apply saved theme
    setTheme(getTheme());

    // Apply saved left sidebar state
    const sidebar = document.getElementById('sidebar');
    if (sidebar) {
      const isMobile = window.innerWidth <= 996;
      applySlidebarState(isMobile ? false : getSidebarState());
    }

    // TOC panel: show toggle button if panel has content, apply saved state
    const tocPanel = document.querySelector('.toc-panel');
    const tocBtn = document.getElementById('toc-btn');
    if (tocPanel && tocPanel.querySelector('.toc-list')) {
      // Panel has headings — make the toggle button visible
      if (tocBtn) tocBtn.classList.add('toc-btn-visible');
      applyTocState(getTocState());
    } else {
      // No headings (index page etc.) — keep button and panel hidden
      if (tocBtn) tocBtn.style.display = 'none';
    }

    // TOC scroll spy
    initTocSpy();

    // Search
    initSearch();

    // Anchor links
    addAnchorLinks();

    // Prism highlight
    if (typeof Prism !== 'undefined') {
      Prism.highlightAll();
    }

    // Sidebar overlay click to close
    const overlay = document.getElementById('sidebar-overlay');
    if (overlay) {
      overlay.addEventListener('click', function () {
        setSidebarState(false);
      });
    }

    // Handle window resize
    window.addEventListener('resize', function () {
      const sb = document.getElementById('sidebar');
      if (!sb) return;
      const isMobile = window.innerWidth <= 996;
      if (!isMobile) {
        sb.classList.remove('open');
        const ov = document.getElementById('sidebar-overlay');
        if (ov) ov.classList.remove('visible');
        applySlidebarState(getSidebarState());
      }
    });
  });

})();
