<script lang="ts">export let title: string; export let action = ''; export let onaction: (() => void) | undefined = undefined;</script>
<div class="section-heading"><h2>{title}</h2>{#if action}<button onclick={onaction}>{action}</button>{/if}</div>
