<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let url: string;
  let qr: string;
  let color = "#ed1e79";

  function getQRCode() {
    invoke("create_svg", { url: url, color: color }).then((message: string) => {
      let blob = new Blob([message], { type: "image/svg+xml" });
      qr = URL.createObjectURL(blob);
    });
  }
</script>

<div class="flex flex-row justify-center my-2">
  <div class="flex flex-col gap-4 w-full">
    <div class="flex flex-row gap-2 justify-center">
      <input
        type="text"
        placeholder="Put URL Here"
        class="input input-bordered w-full max-w-xs"
        bind:value={url}
      />
      <input
        type="color"
        class="input w-full max-w-[5em]"
        on:change={() => console.log(color)}
        bind:value={color}
      />
      <button class="btn btn-primary" on:click={getQRCode}
        >Generate QR Code</button
      >
    </div>
    <div class="flex flex-row justify-center">
      {#if qr}
        <img src={qr} alt="qrcode" class="w-1/2" />
      {/if}
    </div>
  </div>
</div>
