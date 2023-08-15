<script>
    import { Alert, AlertDescription, AlertTitle } from "$components/ui/alert";
    import { alert, launch_alert } from "$lib/stores/app_store";
  import "../app.postcss";
  import { invoke, window } from "@tauri-apps/api"
  import { TauriEvent } from "@tauri-apps/api/event"

  window.getCurrent().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
    invoke('exit');
  })
</script>

{#if $alert != null}
  <div class="fixed top-2 z-50 w-[600px]" style="margin-left: calc((100% - 600px) / 2);">
    <Alert class="w-full bg-background" variant={$alert.type}>
      <AlertTitle>{$alert.title}</AlertTitle>
      <AlertDescription>
        {$alert.content}
      </AlertDescription>
    </Alert>
  </div>
{/if}
<main class="container pt-2">
  <slot />
</main>
