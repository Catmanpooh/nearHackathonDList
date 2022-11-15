<script lang="ts">
  import { FOR_SALE_CATEGORIES, COMMUNITY_CATEGORIES, JOBS_CATEGORIES, HOUSING_CATEGORIES } from "$lib/constants";
</script>

<div class="flex flex-col items-center">
<div class="collapse  w-5/6 my-10">
  <input type="checkbox" class="peer" />
  <div
    class="collapse-title bg-primary text-3xl font-bold text-center text-primary-content peer-checked:bg-secondary peer-checked:text-secondary-content"
  >
    For Sale
  </div>
  <div class="mt-10 collapse-content grid lg:grid-cols-4 md:grid-cols-2 gap-4">
    {#each FOR_SALE_CATEGORIES as forSale}
      <a href="/{"forsale"}/{forSale}" class="text-md font-semibold">{forSale}</a>
    {/each}
  </div>
</div>

<div class="collapse w-5/6 my-10">
    <input type="checkbox" class="peer" />
    <div
      class="collapse-title bg-primary text-3xl font-bold text-center text-primary-content peer-checked:bg-secondary peer-checked:text-secondary-content"
    >
      Community
    </div>
    <div class="mt-10 collapse-content grid lg:grid-cols-4 md:grid-cols-2 gap-4">
      {#each COMMUNITY_CATEGORIES as community}
        <a href="/{"community"}/{community}" class="text-md font-semibold">{community}</a>
      {/each}
    </div>
  </div>

  <div class="collapse w-5/6 my-10">
    <input type="checkbox" class="peer" />
    <div
      class="collapse-title bg-primary text-3xl font-bold text-center text-primary-content peer-checked:bg-secondary peer-checked:text-secondary-content"
    >
      Jobs
    </div>
    <div class="mt-10 collapse-content grid lg:grid-cols-4 md:grid-cols-2 gap-4">
      {#each JOBS_CATEGORIES as job}
        <a href="/{"jobs"}/{job}" class="text-md font-semibold">{job}</a>
      {/each}
    </div>
  </div>

  <div class="collapse w-5/6 my-10">
    <input type="checkbox" class="peer" />
    <div
      class="collapse-title bg-primary text-3xl font-bold text-center text-primary-content peer-checked:bg-secondary peer-checked:text-secondary-content"
    >
      Housing
    </div>
    <div class="mt-10 collapse-content grid lg:grid-cols-4 md:grid-cols-2 gap-4">
      {#each HOUSING_CATEGORIES as housing}
        <a href="/{"housing"}/{housing}" class="text-md font-semibold">{housing}</a>
      {/each}
    </div>
  </div>
</div>