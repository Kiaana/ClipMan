export interface Toast {
    id: number;
    message: string;
    type: 'success' | 'error' | 'info';
}

class ToastStore {
    toasts = $state<Toast[]>([]);
    private counter = 0;

    add(message: string, type: 'success' | 'error' | 'info' = 'info') {
        const id = ++this.counter;
        const toast = { id, message, type };
        this.toasts.push(toast);

        // Auto remove after 2 seconds
        setTimeout(() => {
            this.remove(id);
        }, 2000);
    }

    remove(id: number) {
        this.toasts = this.toasts.filter((t) => t.id !== id);
    }
}

export const toastStore = new ToastStore();
