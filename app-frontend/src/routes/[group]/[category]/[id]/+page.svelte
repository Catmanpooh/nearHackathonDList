<script lang="ts">
  import { walletConnection } from "$lib/nearWallet";
  import { Contract, WalletConnection } from "near-api-js";
  import { CONTRACT_ID } from "$lib/constants";

  /** @type {import('./$types').PageData} */
  export let data: any;

  const { group, item } = data;

  let wallet: WalletConnection;

  walletConnection.subscribe((v: WalletConnection) => (wallet = v));

  let isDisabled = false;
  let successFullyRemoved = false;

  const howLongAgo = (date: string) => {
    let oldDate = new Date(date);
    let newDate = new Date();

    const diffTime = Math.abs(Number(newDate) - Number(oldDate));
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    if (diffTime / (1000 * 60 * 60 * 24) < 0.9) {
      return (diffTime / 60000).toFixed() + " minutes ago";
    } else if (diffDays === 1) {
      return diffDays + " day ago";
    }

    return diffDays + " days ago";
  };

  const removePost = async () => {
    isDisabled = true;

    let methodOptions = {
      viewMethods: [],
      changeMethods: [
        `remove_${group === "forsale" ? "for_sale" : group}_item`,
      ],
    };

    const contract = new Contract(
      wallet?.account(),
      CONTRACT_ID,
      methodOptions
    );

    switch (group) {
      case "forsale":
        await contract.remove_for_sale_item({
          category: item.item_info.category,
          account_id: wallet?.getAccountId(),
          post_id: item.item_info.post_id,
        });
        break;
      case "community":
        await contract.remove_community_item({
          category: item.item_info.category,
          account_id: wallet?.getAccountId(),
          post_id: item.item_info.post_id,
        });
        break;
      case "jobs":
        await contract.remove_jobs_item({
          category: item.item_info.category,
          account_id: wallet?.getAccountId(),
          post_id: item.item_info.post_id,
        });
        break;
      case "housing":
        await contract.remove_housing_item({
          category: item.item_info.category,
          account_id: wallet?.getAccountId(),
          post_id: item.item_info.post_id,
        });
        break;
      default:
        console.log("Error occured");
    }

    isDisabled = false;
    successFullyRemoved = true;
  };
</script>

<div class="flex flex-col justify-center items-center mt-4">
  {#if !successFullyRemoved}
    <h1 class="my-10 text-3xl font-bold">{item.item_info.title}</h1>
    {#if item.item_info.image !== null}
      {#each item.item_info.image as image, i}
        <div class="carousel w-full my-4">
          <div id="item{i}" class="carousel-item w-full">
            <img src={image} alt="" class="w-full" />
          </div>
          <div class="flex justify-center w-full py-2 gap-2">
            <a href="#item{i}" class="btn btn-xs">{i + 1}</a>
          </div>
        </div>
      {/each}
    {/if}

    <p class="text-xl my-4">{item.item_info.description}</p>

    {#each Object.entries(item) as [key, value]}
      {#if !(key === "item_info")}
        {#if value}
          {#if typeof value === "object"}
            {#each Object.entries(value) as [keyInside, valueInside]}
              <p class="btn my-4">
                {#if keyInside === "0"}
                OPEN_HOUSE_BEGINS:
                {:else if keyInside === "1" }
                OPEN_HOUSE_ENDS:
                {:else}
                {keyInside}:
                {/if}
                <span class="mx-2">{valueInside}</span>
              </p>
            {/each}
          {:else}
            <p class="btn my-4">
              {key}:
              <span class="mx-2">{value}</span>
            </p>
          {/if}
        {/if}
      {/if}
    {/each}

    <div class="flex w-1/2 justify-evenly">
      <p>post_id: {item.item_info.post_id}</p>
      <p>posted: {howLongAgo(item.item_info.date)}</p>
    </div>
    {#if item.item_info.creator === wallet.getAccountId()}
      <button
        class="my-8 btn btn-error"
        disabled={isDisabled}
        on:click={() => {
          removePost();
        }}>Remove Post</button
      >
    {/if}
  {:else}
    <h1 class="my-10 text-3xl font-bold">Post has been removed</h1>
  {/if}
</div>
