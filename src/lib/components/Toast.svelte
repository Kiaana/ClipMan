<script lang="ts">
import { toastStore } from '$lib/stores/toast.svelte';
</script>

<div class="toast-container">
  {#each toastStore.toasts as toast (toast.id)}
    <div class="toast" class:success={toast.type === 'success'} class:error={toast.type === 'error'}>
      {toast.message}
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    z-index: 9999;
    pointer-events: none;
  }

  .toast {
    padding: 0.5rem 1rem;
    border-radius: 2rem;
    background-color: #333;
    color: white;
    font-size: 0.875rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    animation: slide-up 0.2s ease-out;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toast.success {
    background-color: #10b981;
  }

  .toast.error {
    background-color: #ef4444;
  }

  @keyframes slide-up {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
