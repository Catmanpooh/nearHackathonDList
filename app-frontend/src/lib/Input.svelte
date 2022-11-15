<script lang="ts">
  export let title: string;
  export let inputName: string;
  export let placeHolder: string;
  export let required: boolean;
  export let inputType: string;
</script>

<div class="form-control w-full max-w-xs">
  <!-- svelte-ignore a11y-label-has-associated-control -->
  <label name={inputName} class="label">
    <span class="label-text">{title}</span>
  </label>
  <input
    type={inputType}
    name={inputName}
    {required}
    placeholder={placeHolder}
    class="input input-bordered w-full max-w-xs"
  />
</div>
