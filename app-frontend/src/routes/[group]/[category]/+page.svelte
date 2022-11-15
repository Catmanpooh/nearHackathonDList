<script lang="ts">
  interface Data {
    group: string;
    category: string;
    items: Array<any>;
  }

  /** @type {import('./$types').PageData} */
  export let data: Data;

  const { group, category, items } = data;
  
  const convertDateToHumanReadable = (date: string) => {
    let newDate = new Date(date);
    return newDate.toDateString().substring(4, 11);
  };
</script>

<div class="flex flex-col justify-center items-center mt-4">
  <h1 class="text-3xl font-bold">{category}</h1>
  {#if items.length >= 1}
    {#each items as item}
      <div class="flex w-1/2 mt-10 justify-evenly">
        <p>{convertDateToHumanReadable(item.item_info.date)}</p>
        <a href="/{group}/{category}/{item.item_info.post_id}">{item.item_info.title}</a>
        {#if item.item_info.location}
          <p>({item.item_info.location})</p>
        {/if}
      </div>
    {/each}
  {:else}
    <h2 class="text-xl font-semibold my-10">Nothing listed yet</h2>
    <p class="text-lg font-semibold">Be the First!!</p>
  {/if}
</div>
