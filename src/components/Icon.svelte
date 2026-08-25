<script lang="ts">
  export let name: 'home' | 'search' | 'library' | 'settings' | 'play' | 'pause' | 'next' | 'previous' | 'heart' | 'more' | 'plus' | 'back' | 'shuffle' | 'repeat' | 'queue' | 'device' | 'sort' | 'grid' | 'music' | 'chevron' | 'check';
  export let size = 24;
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
  {#if name === 'home'}<path d="m3 11 9-8 9 8v9a1 1 0 0 1-1 1h-5v-7H9v7H4a1 1 0 0 1-1-1z" />
  {:else if name === 'search'}<circle cx="11" cy="11" r="7" /><path d="m20 20-4-4" />
  {:else if name === 'library'}<path d="M4 4h3v16H4zm6 0h3v16h-3zm6 1 3-1 3 15-3 1z" />
  {:else if name === 'settings'}<circle cx="12" cy="12" r="3" /><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1-2.8 2.8-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.6v.2h-4V21a1.7 1.7 0 0 0-1-1.6 1.7 1.7 0 0 0-1.9.3l-.1.1L4.2 17l.1-.1a1.7 1.7 0 0 0 .3-1.9A1.7 1.7 0 0 0 3 14H2.8v-4H3a1.7 1.7 0 0 0 1.6-1 1.7 1.7 0 0 0-.3-1.9L4.2 7 7 4.2l.1.1A1.7 1.7 0 0 0 9 4.6 1.7 1.7 0 0 0 10 3v-.2h4V3a1.7 1.7 0 0 0 1 1.6 1.7 1.7 0 0 0 1.9-.3l.1-.1L19.8 7l-.1.1a1.7 1.7 0 0 0-.3 1.9 1.7 1.7 0 0 0 1.6 1h.2v4H21a1.7 1.7 0 0 0-1.6 1z" />
  {:else if name === 'play'}<path d="m8 5 11 7-11 7z" fill="currentColor" />
  {:else if name === 'pause'}<path d="M8 5v14M16 5v14" />
  {:else if name === 'next'}<path d="m5 4 10 8-10 8zM19 5v14" />
  {:else if name === 'previous'}<path d="m19 20-10-8 10-8zM5 5v14" />
  {:else if name === 'heart'}<path d="M20.8 4.6a5.5 5.5 0 0 0-7.8 0L12 5.7l-1.1-1.1a5.5 5.5 0 0 0-7.8 7.8L12 21l8.9-8.6a5.5 5.5 0 0 0-.1-7.8z" />
  {:else if name === 'more'}<circle cx="5" cy="12" r="1" fill="currentColor" /><circle cx="12" cy="12" r="1" fill="currentColor" /><circle cx="19" cy="12" r="1" fill="currentColor" />
  {:else if name === 'plus'}<path d="M12 5v14M5 12h14" />
  {:else if name === 'back'}<path d="m15 18-6-6 6-6" />
  {:else if name === 'shuffle'}<path d="M16 3h5v5M4 20 21 3M21 16v5h-5M15 15l6 6M4 4l5 5" />
  {:else if name === 'repeat'}<path d="m17 2 4 4-4 4M3 11V9a3 3 0 0 1 3-3h15M7 22l-4-4 4-4m14-1v2a3 3 0 0 1-3 3H3" />
  {:else if name === 'queue'}<path d="M4 6h12M4 12h12M4 18h8M19 15v6m-3-3h6" />
  {:else if name === 'device'}<rect x="3" y="5" width="11" height="14" rx="2" /><path d="M18 8h3v10h-3M7 16h3" />
  {:else if name === 'sort'}<path d="M8 6h12M8 12h9M8 18h6M4 4v16m0 0-2-2m2 2 2-2" />
  {:else if name === 'grid'}<rect x="3" y="3" width="7" height="7" /><rect x="14" y="3" width="7" height="7" /><rect x="3" y="14" width="7" height="7" /><rect x="14" y="14" width="7" height="7" />
  {:else if name === 'music'}<path d="M9 18V5l10-2v13" /><circle cx="6" cy="18" r="3" /><circle cx="16" cy="16" r="3" />
  {:else if name === 'chevron'}<path d="m9 18 6-6-6-6" />
  {:else if name === 'check'}<path d="m5 12 4 4L19 6" />{/if}
</svg>
