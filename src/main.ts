import './style.css';
import App from './App.svelte';
import { mount } from 'svelte';

export default mount(App, { target: document.getElementById('app')! });

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  void navigator.serviceWorker.register('/sw.js');
}
