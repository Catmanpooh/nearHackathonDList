import { error } from "@sveltejs/kit";
import { walletConnection } from "$lib/nearWallet";
import { Contract, WalletConnection } from "near-api-js";
import { CONTRACT_ID } from "$lib/constants";

/** @type {import('./$types').PageLoad} */
export async function load({ params }: any) {
  let wallet: WalletConnection;

  walletConnection.subscribe((v: WalletConnection) => (wallet = v));

  const getItemsFromContract = async () => {
    let getItems;

    let group = params.group === "forsale" ? "for_sale" : params.group;

    let methodOptions = {
      viewMethods: [`get_all_${group}_items`],
      changeMethods: [],
    };

    const contract = new Contract(
      wallet?.account(),
      CONTRACT_ID,
      methodOptions
    );

    switch (params.group) {
      case "forsale":
        getItems = await contract.get_all_for_sale_items({
          category: params.category,
          // category: "computers",
        });
        console.log(getItems);
        
        break;
      case "community":
        getItems = await contract.get_all_community_items({
          category: params.category,
        });
        break;
      case "jobs":
        getItems = await contract.get_all_jobs_items({
          category: params.category,
        });
        break;
      case "housing":
        getItems = await contract.get_all_housing_items({
          category: params.category,
        });
        break;
      default:
        throw error(404, "Not Found");
    }

    return getItems;
  };

  return {
    group: params.group,
    category: params.category,
    items: await getItemsFromContract(),
  };
}
