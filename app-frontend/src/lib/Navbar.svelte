<script lang="ts">
  import { CONTRACT_ID } from "$lib/constants";
  import type { WalletConnection } from "near-api-js";
  import { onMount } from "svelte";
  import { themeChange } from "theme-change";

  export let wallet: WalletConnection;

  onMount(() => {
    themeChange(false);
    // 👆 false parameter is required for svelte
  });

  const themes = [
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
  ];

  const signInFlow = () => {
    wallet?.requestSignIn({ contractId: CONTRACT_ID });
  };

  const signOutFlow = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };
</script>

<div class="navbar bg-base-100 my-2">
  <div class="navbar-start">
    <a href="/" class="btn btn-ghost normal-case text-2xl">dList</a>
  </div>

  <div class="navbar-end">
    <select class="select w-42 max-w-xs mx-4" data-choose-theme>
      {#each themes as theme}
        <option value={theme}>{theme}</option>
      {/each}
    </select>

    {#if wallet?.isSignedIn()}
      <ul class="menu menu-horizontal p-0">
        <li><a class="text-lg font-bold mx-4" href="/post">Post</a></li>
        <li>
          <button class="btn btn-primary mr-2" on:click={signOutFlow}
            >Sign out {wallet?.getAccountId()}</button
          >
        </li>
      </ul>
    {:else}
      <button on:click={signInFlow} class="btn btn-primary">Sign In</button>
    {/if}
  </div>
</div>
