const browser = typeof window !== 'undefined';

type Theme = 'light' | 'dark' | 'light-pink' | 'system';

function createThemeStore() {
    let theme = $state<Theme>('system');

    // Initialize from localStorage if available
    if (browser) {
        const stored = localStorage.getItem('theme') as Theme;
        if (stored && ['light', 'dark', 'light-pink', 'system'].includes(stored)) {
            theme = stored;
        }
    }

    // Effect removed - handled in component

    return {
        get current() { return theme; },
        setTheme: (newTheme: Theme) => { theme = newTheme; },
        toggle: () => {
            if (theme === 'light') theme = 'dark';
            else if (theme === 'dark') theme = 'light-pink';
            else if (theme === 'light-pink') theme = 'light'; // Cycle back to light for now, or system? Let's do 3-way cycle + system is separate
            else theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'light' : 'dark';
        }
    };
}

export const themeStore = createThemeStore();
