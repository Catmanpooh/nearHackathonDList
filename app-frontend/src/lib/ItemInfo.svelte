

<div class="flex justify-between w-5/6">
    <Select
      title="Pick the group"
      selectName="group"
      disabled={false}
      required={true}
      bind:value
      postTypes={postingTypes}
    />
    <Select
      title="Pick the category"
      selectName="category"
      disabled={catDisabled}
      required={true}
      bind:value={catValue}
      postTypes={catArray}
    />
  </div>
  <Input
    title="Enter Post Title"
    inputName="title"
    placeHolder="Enter Title"
    inputType="text"
    required={true}
  />
  <div class="form-control w-full max-w-xs my-8">
    <textarea
      name="description"
      class="textarea textarea-bordered"
      placeholder="Enter Description"
    />
  </div>
  <div class="flex justify-between">
    <Input
      title="Enter The Location"
      inputName="location"
      placeHolder="Enter Location"
      inputType="text"
      required={false}
    />
    <Input
      title="Enter Price"
      inputName="price"
      placeHolder="Enter Price"
      inputType="number"
      required={false}
    />
    <Input
      title="Enter Postal Code"
      inputName="postal_code"
      placeHolder="Enter Postal Code"
      inputType="number"
      required={false}
    />
    <Input
      title="Enter The City or Neighborhood"
      inputName="city_or_neighborhood"
      placeHolder="Enter  City or Neighborhood"
      inputType="text"
      required={false}
    />
  </div>