<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMount } from 'svelte';

let permissionStatus = $state<'checking' | 'granted' | 'denied'>('checking');
let errorMessage = $state('');

onMount(async () => {
    await checkPermission();
});

async function checkPermission() {
    try {
        permissionStatus = 'checking';
        const result = await invoke<string>('check_clipboard_permission');

        if (result === 'granted') {
            permissionStatus = 'granted';
        } else {
            permissionStatus = 'denied';
            errorMessage = result;
        }
    } catch (err) {
        permissionStatus = 'denied';
        errorMessage = String(err);
    }
}
</script>

{#if permissionStatus === 'denied'}
    <div class="permission-banner">
        <div class="banner-content">
            <div class="icon">⚠️</div>
            <div class="message">
                <h3>需要辅助功能权限</h3>
                <p>ClipMan 需要辅助功能权限才能监控剪切板变化。</p>
                <details>
                    <summary>如何授予权限？</summary>
                    <ol>
                        <li>打开 <strong>系统偏好设置</strong> → <strong>安全性与隐私</strong></li>
                        <li>选择 <strong>隐私</strong> 标签</li>
                        <li>在左侧列表中选择 <strong>辅助功能</strong></li>
                        <li>点击左下角 🔒 解锁</li>
                        <li>添加 <strong>终端</strong> 或 <strong>iTerm</strong>（运行此应用的终端）</li>
                        <li>勾选该应用</li>
                        <li>重启 ClipMan</li>
                    </ol>
                    {#if errorMessage}
                        <p class="error-detail">错误详情: {errorMessage}</p>
                    {/if}
                </details>
                <button onclick={checkPermission}>🔄 重新检查</button>
            </div>
        </div>
    </div>
{/if}

<style>
.permission-banner {
    background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
    border: 2px solid #f59e0b;
    border-radius: 8px;
    margin: 1rem;
    padding: 1rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.banner-content {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
}

.icon {
    font-size: 2rem;
    flex-shrink: 0;
}

.message {
    flex: 1;
}

.message h3 {
    margin: 0 0 0.5rem 0;
    color: #92400e;
    font-size: 1.1rem;
}

.message p {
    margin: 0 0 1rem 0;
    color: #78350f;
    font-size: 0.95rem;
}

details {
    background: rgba(255, 255, 255, 0.6);
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
}

summary {
    cursor: pointer;
    font-weight: 600;
    color: #92400e;
    user-select: none;
}

summary:hover {
    color: #78350f;
}

ol {
    margin: 1rem 0 0 0;
    padding-left: 1.5rem;
}

ol li {
    margin: 0.5rem 0;
    color: #78350f;
}

strong {
    color: #92400e;
}

.error-detail {
    margin-top: 1rem;
    padding: 0.5rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 4px;
    font-size: 0.85rem;
    color: #991b1b;
    font-family: monospace;
}

button {
    background: #f59e0b;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 600;
    transition: background 0.2s;
}

button:hover {
    background: #d97706;
}
</style>
