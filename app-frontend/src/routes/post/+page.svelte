<script lang="ts">
  import { walletConnection } from "$lib/nearWallet";
  import { Contract, WalletConnection } from "near-api-js";
  import { CONTRACT_ID } from "$lib/constants";
  import {
    FOR_SALE_CATEGORIES,
    COMMUNITY_CATEGORIES,
    JOBS_CATEGORIES,
    HOUSING_CATEGORIES,
  } from "$lib/constants";
  import Select from "$lib/Select.svelte";
  import Input from "$lib/Input.svelte";

  let wallet: WalletConnection;

  walletConnection.subscribe((v: WalletConnection) => (wallet = v));

  const category = [
    FOR_SALE_CATEGORIES,
    COMMUNITY_CATEGORIES,
    JOBS_CATEGORIES,
    HOUSING_CATEGORIES,
  ];

  const postingTypes = [
    "Job Offered",
    "Housing Wanted",
    "Housing Offered",
    "For Sale by Owner",
    "For Sale by Dealer",
    "Wanted by Owner",
    "Wanted by Dealer",
    "Community",
  ];

  const conditionItems = [
    "New",
    "LikeNew",
    "Excellent",
    "Good",
    "Fair",
    "Salvage",
  ];
  const communityItems = [
    "Garage Sale",
    "Class or Event",
    "Lost or Found",
    "RideShare",
  ];

  const employmentItems = [
    "FullTime",
    "PartTime",
    "ContractWork",
    "EmployeeChoice",
  ];

  const housingTypeItems = [
    "Apartment",
    "Condo",
    "CottageOrCabin",
    "Duplex",
    "Flat",
    "House",
    "InLaw",
    "Loft",
    "TownHouse",
    "Manufactured",
    "AssistedLiving",
    "Land",
  ];

  const housingPerTimeItems = ["Day", "Week", "Month"];

  let value: string | undefined;
  let catArray: Array<string> = [];
  let catValue: string | undefined;
  let catDisabled = true;
  let checkDisplayItems: number | undefined;
  let conditionValue: string | undefined;
  let communityValue: string | undefined;
  let displayCommunity: number | null = null;
  let employmentValue: string | undefined;
  let housingTypeValue: string | undefined;
  let housingPerTimeValue: string | undefined;
  let isDisabled = false;
  let successFullyRemoved = false;

  $: if (value !== undefined) handleCategory();
  $: if (communityValue !== undefined) handleCommunity();

  const handleCommunity = () => {
    if (communityValue === communityItems[0]) {
      displayCommunity = 0;
    } else if (communityValue === communityItems[1]) {
      displayCommunity = 1;
    } else if (communityValue === communityItems[2]) {
      displayCommunity = 2;
    } else {
      displayCommunity = 3;
    }
  };

  const handleCategory = () => {
    if (value === postingTypes[1] || value === postingTypes[2]) {
      catArray = category[3];
      checkDisplayItems = 3;
    } else if (
      value === postingTypes[3] ||
      value === postingTypes[4] ||
      value === postingTypes[5] ||
      value === postingTypes[6]
    ) {
      catArray = category[0];
      checkDisplayItems = 0;
    } else if (value === postingTypes[7]) {
      catArray = category[1];
      checkDisplayItems = 1;
    } else {
      catArray = category[2];
      checkDisplayItems = 2;
    }

    catDisabled = false;
  };

  const handleSubmit = async (e: Event) => {
    isDisabled = true;

    const formData = Object.fromEntries(new FormData(e.target));
    console.log(formData);

    if (formData.group === undefined || formData.category === undefined) {
      return alert("Group and Category needs to be selected");
    }
    if (formData.title === undefined || formData.description === undefined) {
      return alert("Title and Description need to be entered");
    }

    const firstChar = formData.group?.charAt(0).toLowerCase();

    const item_info = {
      creator: wallet.getAccountId(),
      post_id: Number(Math.floor(Math.random() * 3654651989125135)),
      date: Date.now(),
      category: formData.category.toString().trim(),
      title: formData.title.toString(),
      description: formData.description.toString(),
      image: null, // for now might have time to update
      location: formData.location || null,
      price: Number(formData.price) || null,
      postal_code: Number(formData.postal_code) || null,
      city_or_neighborhood: formData.city_or_neighborhood.toString() || null,
    };

    type MethodOptions = {
      viewMethods: Array<string>;
      changeMethods: Array<string>;
    };
    let methodOptions: MethodOptions;

    let contract: Contract;

    if (firstChar === "j") {
      methodOptions = {
        viewMethods: [],
        changeMethods: ["set_jobs_item"],
      };
      contract = new Contract(wallet?.account(), CONTRACT_ID, methodOptions);
      const jobs_info_in = {
        item_info: item_info,
        employment_type: formData.employment_type.toString() || null,
        job_title: formData.job_title || null,
        compensation: Number(formData.compensation) || null,
        company_name: formData.company_name || null,
      };

      try {
        await contract.set_jobs_item(
          { jobs_info_in },
          "30000000000000",
          "1000000000000000000000000"
        );
      } catch (err) {
        alert("Post was unsuccessful. Try again");
      }

      console.log(jobs_info_in);
    } else if (firstChar === "h") {
      methodOptions = {
        viewMethods: [],
        changeMethods: ["set_housing_item"],
      };
      contract = new Contract(wallet?.account(), CONTRACT_ID, methodOptions);
      let open_house_dates: Array<string> | null = null;

      let begin = formData.open_house_dates_begin || null;
      let end = formData.open_house_dates_end || null;

      if (begin || end) {
        open_house_dates = [
          formData.open_house_dates_begin.toString(),
          formData.open_house_dates_end.toString(),
        ];
      }

      const housing_info_in = {
        item_info: item_info,
        rent: Number(formData.rent) || null,
        per_time_range: formData.per_time_range || null,
        sqft: Number(formData.sqft) || null,
        pet: (formData.pet === "on" ? true : false) || null,
        air_conditioning:
          (formData.air_conditioning === "on" ? true : false) || null,
        private_room: (formData.private_room === "on" ? true : false) || null,
        housing_type: formData.housing_type || null,
        laundry: (formData.laundry === "on" ? true : false) || null,
        parking: (formData.parking === "on" ? true : false) || null,
        available_date: formData.available_date || null,
        open_house_dates: open_house_dates,
      };

      try {
        await contract.set_housing_item({ housing_info_in });
      } catch (err) {
        alert("Post was unsuccessful. Try again");
      }

      console.log(housing_info_in);
    } else if (firstChar === "f" || firstChar === "w") {
      methodOptions = {
        viewMethods: [],
        changeMethods: ["set_for_sale_item"],
      };
      contract = new Contract(wallet?.account(), CONTRACT_ID, methodOptions);
      const for_sale_info_in = {
        item_info: item_info,
        make_or_manufacturer: formData.make_or_manufacturer || null,
        model_name_or_number: formData.model_name_or_number || null,
        size_dimensions: formData.size_dimensions || null,
        condition: formData.condition || null,
      };
      try {
        await contract.set_for_sale_item({ for_sale_info_in });
      } catch (err) {
        alert("Post was unsuccessful. Try again");
      }
      console.log(for_sale_info_in);
    } else if (firstChar === "c") {
      methodOptions = {
        viewMethods: [],
        changeMethods: ["set_community_item"],
      };
      contract = new Contract(wallet?.account(), CONTRACT_ID, methodOptions);
      type GarageSale = {
        garage_sale_start_time: string | null;
        garage_sale_dates: Array<string> | null;
      };
      type ClassOrEvent = {
        event_venue: string | null;
        event_start_date: string | null;
        event_duration: number | null;
        event_features: Array<string>;
      };

      let garage_sale: GarageSale | null = null;
      let class_or_event: ClassOrEvent | null = null;

      if (displayCommunity === 0) {
        let garage_sale_dates: Array<string> | null = null;
        let begin = formData.garage_sale_dates_begin || null;
        let end = formData.garage_sale_dates_end || null;
        if (begin || end) {
          garage_sale_dates = [
            formData.garage_sale_dates_begin.toString(),
            formData.garage_sale_dates_end.toString(),
          ];
        }
        garage_sale = {
          garage_sale_start_time:
            formData.garage_sale_start_time.toString() || null,
          garage_sale_dates: garage_sale_dates,
        };
      }

      if (displayCommunity === 1) {
        let event_features =
          formData.event_features.toString().split(",") || null;
        class_or_event = {
          event_venue: formData.event_venue.toString() || null,
          event_start_date: formData.event_start_date.toString() || null,
          event_duration: Number(formData.event_duration) || null,
          event_features: event_features,
        };
      }

      const community_info_in = {
        item_info: item_info,
        garage_sale: garage_sale,
        class_or_event: class_or_event,
        lost_or_found: (formData.lost_or_found === "on" ? true : false) || null,
        rideshare: (formData.rideshare === "on" ? true : false) || null,
      };
      try {
        await contract.set_community_item({ community_info_in });
      } catch (err) {
        alert("Post was unsuccessful. Try again");
      }

      console.log(community_info_in);
    } else {
      return alert("Post was not entered");
    }

    isDisabled = false;
    successFullyRemoved = true;
  };
</script>

<div class="lg:container mx-auto h-full w-full p-8">
  {#if checkDisplayItems === 2}
    <h1 class="text-3xl font-medium text-center my-2">Create A Post | Cost To Post Job Is 1 Near</h1>
  {:else}
    <h1 class="text-3xl font-medium text-center my-2">Create A Post</h1>
  {/if}
  <form
    on:submit|preventDefault={handleSubmit}
    class="flex flex-col items-center w-full justify-center"
  >
    <div class="flex flex-wrap gap-x-8 gap-y-8  w-full my-8">
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
      <Input
        title="Enter Post Title"
        inputName="title"
        placeHolder="Enter Title"
        inputType="text"
        required={true}
      />
      <Input
        title="Enter Price"
        inputName="price"
        placeHolder="Enter Price"
        inputType="number"
        required={false}
      />
      <Input
        title="Enter The Location"
        inputName="location"
        placeHolder="Enter Location"
        inputType="text"
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
    <div class="form-control w-full my-8">
      <textarea
        name="description"
        class="textarea textarea-bordered w-full h-64"
        placeholder="Enter Description"
        required
      />
    </div>

    <div class="flex flex-wrap gap-x-8 gap-y-8  w-full my-8">
      {#if checkDisplayItems === 0}
        <Input
          title="Enter The Make or Manufacturer"
          inputName="make_or_manufacturer"
          placeHolder="Enter Make or Manufacturer"
          inputType="text"
          required={false}
        />
        <Input
          title="Enter Model Name or Number"
          inputName="model_name_or_number"
          placeHolder="Enter Model Name or Number"
          inputType="text"
          required={false}
        />
        <Input
          title="Enter Size Dimensions"
          inputName="size_dimensions"
          placeHolder="Enter Size Dimensions"
          inputType="text"
          required={false}
        />
        <Select
          title="Pick the Condition"
          selectName="condition"
          disabled={false}
          required={false}
          bind:value={conditionValue}
          postTypes={conditionItems}
        />
      {:else if checkDisplayItems === 1}
        <Select
          title="Pick the Community Type"
          selectName="community"
          disabled={false}
          required={false}
          bind:value={communityValue}
          postTypes={communityItems}
        />
        {#if displayCommunity === 0}
          <Input
            title="Enter Garage Sale Start Time"
            inputName="garage_sale_start_time"
            placeHolder="Start Time For Garage Sale"
            inputType="time"
            required={false}
          />
          <Input
            title="Enter Begin Date For Sale"
            inputName="garage_sale_dates_begin"
            placeHolder="Begin Date"
            inputType="date"
            required={false}
          />
          <Input
            title="Enter End Date For Sale"
            inputName="garage_sale_dates_end"
            placeHolder="End Date"
            inputType="date"
            required={false}
          />
        {:else if displayCommunity === 1}
          <Input
            title="Enter Event Venue"
            inputName="event_venue"
            placeHolder="Enter Venue"
            inputType="text"
            required={false}
          />
          <Input
            title="Enter Start Date Event"
            inputName="event_start_date"
            placeHolder="Start Date"
            inputType="date"
            required={false}
          />
          <Input
            title="Enter Event Duration"
            inputName="event_duration"
            placeHolder="Enter Duration"
            inputType="number"
            required={false}
          />
          <Input
            title="Enter Event Features"
            inputName="event_features"
            placeHolder="Enter Features Separate by commas"
            inputType="text"
            required={false}
          />
        {:else if displayCommunity === 2}
          <Input
            title="Enter Lost and Found"
            inputName="lost_or_found"
            placeHolder="Enter Lost and Found"
            inputType="checkbox"
            required={false}
          />
        {:else if displayCommunity === 3}
          <Input
            title="Enter Ride Share"
            inputName="rideshare"
            placeHolder="Enter RideShare"
            inputType="checkbox"
            required={false}
          />
        {/if}
      {:else if checkDisplayItems === 2}
        <Input
          title="Enter The Company Name"
          inputName="company_name"
          placeHolder="Enter Company Name"
          inputType="text"
          required={false}
        />
        <Input
          title="Enter Compensation"
          inputName="compensation"
          placeHolder="Enter Compensation"
          inputType="number"
          required={false}
        />
        <Input
          title="Enter Job Title"
          inputName="job_title"
          placeHolder="Enter Job Title"
          inputType="text"
          required={false}
        />
        <Select
          title="Pick the Employment Type"
          selectName="employment_type"
          disabled={false}
          required={false}
          bind:value={employmentValue}
          postTypes={employmentItems}
        />
      {:else if checkDisplayItems === 3}
        <Input
          title="Becomes Available"
          inputName="available_date"
          placeHolder="Enter Company Name"
          inputType="date"
          required={false}
        />
        <Input
          title="Enter Begin Date Open House"
          inputName="open_house_dates_begin"
          placeHolder="Begin Date"
          inputType="date"
          required={false}
        />
        <Input
          title="Enter End Date Open House"
          inputName="open_house_dates_end"
          placeHolder="End Date"
          inputType="date"
          required={false}
        />
        <Input
          title="Enter Rent Amount"
          inputName="rent"
          placeHolder="Enter Rent Amount"
          inputType="number"
          required={false}
        />
        <Input
          title="Enter Sqft Amount"
          inputName="sqft"
          placeHolder="Enter Sqft Amount"
          inputType="number"
          required={false}
        />
        <Select
          title="Pick the Housing Type"
          selectName="housing_type"
          disabled={false}
          required={false}
          bind:value={housingTypeValue}
          postTypes={housingTypeItems}
        />
        <Select
          title="Pick the Housing Type"
          selectName="per_time_range"
          disabled={false}
          required={false}
          bind:value={housingPerTimeValue}
          postTypes={housingPerTimeItems}
        />
        <Input
          title="Allow Pets"
          inputName="pet"
          placeHolder="Pets"
          inputType="checkbox"
          required={false}
        />
        <Input
          title="Has Air Conditioning"
          inputName="air_conditioning"
          placeHolder="Air"
          inputType="checkbox"
          required={false}
        />
        <Input
          title="Is Private Room"
          inputName="private_room"
          placeHolder="Private Room"
          inputType="checkbox"
          required={false}
        />
        <Input
          title="Has Laundry"
          inputName="laundry"
          placeHolder="Laundry"
          inputType="checkbox"
          required={false}
        /><Input
          title="Has Parking"
          inputName="parking"
          placeHolder="Parking"
          inputType="checkbox"
          required={false}
        />
      {/if}
    </div>
    <div class="my-8" />

    <button type="submit" class="btn btn-primary" disabled={isDisabled}
      >submit</button
    >
  </form>
</div>
