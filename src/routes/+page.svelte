<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let url;
  let qr;

  function getQRCode() {
    invoke("create_svg", { invokeMessage: url }).then((message: string) => {
      let blob = new Blob([message], { type: "image/svg+xml" });
      qr = URL.createObjectURL(blob);
    });
  }
</script>

<div class="flex flex-row justify-center my-2">
  <div class="flex flex-col gap-4">
    <div class="flex flex-row gap-2">
      <input
        type="text"
        placeholder="Put URL Here"
        class="input input-bordered w-full max-w-xs"
        bind:value={url}
      />
      <button class="btn btn-primary" on:click={getQRCode}
        >Generate QR Code</button
      >
    </div>
    <div class="flex flex-row justify-center">
      <img src={qr} alt="qrcode" />
    </div>
  </div>
</div>
