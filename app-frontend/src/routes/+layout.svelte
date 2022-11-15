<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { WalletConnection, connect, keyStores } from "near-api-js";
  import Navbar from "$lib/Navbar.svelte";
  import { walletConnection } from "$lib/nearWallet";

  let wallet: WalletConnection;

  walletConnection.subscribe((v) => {
    wallet = v;
  });


  onMount(async () => {
    const connectionConfig = {
      networkId: "testnet",
      keyStore: new keyStores.BrowserLocalStorageKeyStore(),
      nodeUrl: "https://rpc.testnet.near.org",
      //   walletUrl: "https://wallet.testnet.near.org",
      walletUrl: "https://testnet.mynearwallet.com",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org",
    };
    //   connect to NEAR
    const nearConnection = await connect(connectionConfig);

    //   create wallet connection
    let wallet = new WalletConnection(nearConnection, "daniels-list-near");
    walletConnection.update(() => wallet);
  });
</script>

<Navbar {wallet} />
<slot />
