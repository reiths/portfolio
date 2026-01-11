<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import DropZone from "$lib/DropZone.svelte";

    let isProcessing = false;
    let error: string | null = null;
    let success: string | null = null;

    async function handleDrop(paths: string[]) {
        isProcessing = true;
        error = null;
        success = null;
        try {
            const result = await invoke<string>("open_pdf", { paths });
            success = result;
        } catch (err) {
            error = err instanceof Error ? err.message : String(err);
            console.error("PDF processing failed:", err);
        } finally {
            isProcessing = false;
        }
    }
</script>

<main class="container">
    <DropZone onDrop={handleDrop}>
        {#if isProcessing}
            <p class="status">Processing...</p>
        {:else if success}
            <p class="success">{success}</p>
        {:else if error}
            <p class="error">{error}</p>
        {:else}
            <p>Drag and drop PDF files here</p>
        {/if}
    </DropZone>
</main>

<style>
    main {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
    }

    .status {
        margin-top: 1rem;
        color: #2196f3;
        font-weight: 500;
    }

    .success {
        margin-top: 1rem;
        color: #4caf50;
        font-weight: 500;
    }

    .error {
        margin-top: 1rem;
        color: #f44336;
        font-weight: 500;
    }
</style>
