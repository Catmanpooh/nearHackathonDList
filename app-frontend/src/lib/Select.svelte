<script lang="ts">
  export let title: string;
  export let selectName: string;
  export let disabled: boolean;
  export let required: boolean;
  export let postTypes: Array<string>;
  export let value: string | undefined;
</script>

<div class="form-control w-full max-w-xs">
  <!-- svelte-ignore a11y-label-has-associated-control -->
  <label name={selectName} class="label">
    <span class="label-text">{title}</span>
  </label>
  <select
    disabled={disabled}
    bind:value={value}
    name={selectName}
    required={required}
    class="select select-bordered"
  >
    <option disabled selected value={undefined}>Pick one</option>
    {#each postTypes as postType}
      <option value={postType}>{postType}</option>
    {/each}
  </select>
</div>
