/**
 * sidebar.js - File tree component for md-viewer folder mode
 * Handles: folder expand/collapse, active file highlighting, state persistence
 */

(function () {
  'use strict';

  const TREE_STATE_KEY = 'mdv-tree-state';

  /* ── Load / Save folder open states ─────────────────────────── */
  function loadTreeState() {
    try {
      return JSON.parse(localStorage.getItem(TREE_STATE_KEY) || '{}');
    } catch (_) { return {}; }
  }

  function saveTreeState(state) {
    try { localStorage.setItem(TREE_STATE_KEY, JSON.stringify(state)); } catch (_) {}
  }

  /* ── Build SVG icons ─────────────────────────────────────────── */
  function iconFolder(open) {
    return open
      ? '<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>'
      : '<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>';
  }

  function iconFile() {
    return '<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>';
  }

  function iconChevron(open) {
    return open
      ? '<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="6 9 12 15 18 9"/></svg>'
      : '<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="9 18 15 12 9 6"/></svg>';
  }

  /* ── Render tree ─────────────────────────────────────────────── */
  function renderTree(container, nodes, depth, state, currentPath) {
    if (!nodes || !nodes.length) return;

    nodes.forEach(function (node) {
      const item = document.createElement('div');

      if (node.type === 'folder') {
        const isOpen = state[node.path] !== false; // default open
        item.className = 'tree-item folder';
        item.setAttribute('data-depth', depth);
        item.setAttribute('data-folder', node.path);
        item.innerHTML = `<span style="display:flex;align-items:center;gap:3px;margin-right:4px">${iconChevron(isOpen)}${iconFolder(isOpen)}</span><span>${escHtml(node.name)}</span>`;

        const children = document.createElement('div');
        children.className = 'tree-children' + (isOpen ? '' : ' collapsed');
        children.style.maxHeight = isOpen ? '9999px' : '0';

        item.addEventListener('click', function (e) {
          e.stopPropagation();
          const nowOpen = !children.classList.contains('collapsed');
          children.classList.toggle('collapsed', nowOpen);
          children.style.maxHeight = nowOpen ? '0' : '9999px';
          state[node.path] = !nowOpen;
          saveTreeState(state);
          item.querySelector('span span:first-child') && (item.querySelector('.tree-item.folder span') );
          item.innerHTML = `<span style="display:flex;align-items:center;gap:3px;margin-right:4px">${iconChevron(!nowOpen)}${iconFolder(!nowOpen)}</span><span>${escHtml(node.name)}</span>`;
        });

        renderTree(children, node.children, depth + 1, state, currentPath);
        container.appendChild(item);
        container.appendChild(children);

      } else {
        // file node
        const isActive = currentPath && (node.path === currentPath || '/' + node.path === currentPath);
        item.className = 'tree-item' + (isActive ? ' active' : '');
        item.setAttribute('data-depth', depth);
        item.innerHTML = `${iconFile()}<span>${escHtml(node.name)}</span>`;
        item.setAttribute('role', 'link');
        item.setAttribute('tabindex', '0');
        item.addEventListener('click', function () {
          window.location.href = '/' + node.path;
        });
        item.addEventListener('keydown', function (e) {
          if (e.key === 'Enter' || e.key === ' ') window.location.href = '/' + node.path;
        });
        container.appendChild(item);
      }
    });
  }

  /* ── Helpers ─────────────────────────────────────────────────── */
  function escHtml(str) {
    return String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  /* ── Init ────────────────────────────────────────────────────── */
  document.addEventListener('DOMContentLoaded', function () {
    const container = document.getElementById('file-tree');
    if (!container || typeof FILE_TREE === 'undefined') return;

    const state = loadTreeState();
    const currentPath = window.CURRENT_PAGE || '';

    renderTree(container, FILE_TREE, 1, state, currentPath);

    // Scroll active item into view in sidebar
    const active = container.querySelector('.tree-item.active');
    if (active) {
      active.scrollIntoView({ block: 'nearest' });
    }
  });

})();
