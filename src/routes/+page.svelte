<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"
</script>

<div class="flex flex-row justify-center my-2">
  <div class="flex flex-col">
    <div class="flex flex-row gap-2">
      <input
        type="text"
        placeholder="Put URL Here"
        class="input input-bordered w-full max-w-xs"
      />
      <button class="btn btn-primary" on:click={()=> {
        invoke("create_svg")
      }}>Generate QR Code</button>
    </div>
  </div>
</div>
