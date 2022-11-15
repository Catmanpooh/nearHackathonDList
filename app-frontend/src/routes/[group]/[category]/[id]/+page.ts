import { error } from "@sveltejs/kit";
import { walletConnection } from "$lib/nearWallet";
import { Contract, WalletConnection } from "near-api-js";
import { CONTRACT_ID } from "$lib/constants";

/** @type {import('./$types').PageLoad} */
export async function load({ params }: any) {
  let wallet: WalletConnection;

  walletConnection.subscribe((v: WalletConnection) => (wallet = v));

  const getItemFromContract = async () => {
    let getItem;

    let group = params.group === "forsale" ? "for_sale" : params.group;

    let methodOptions = {
      viewMethods: [`get_${group}_item`],
      changeMethods: [],
    };

    const contract = new Contract(
      wallet?.account(),
      CONTRACT_ID,
      methodOptions
    );

    switch (params.group) {
      case "forsale":
        getItem = await contract.get_for_sale_item({
          category: params.category,
          // category: "atv",
          post_id: Number(params.id),
        });

        break;
      case "community":
        getItem = await contract.get_community_item({
          category: params.category,
          post_id: Number(params.id),
        });
        break;
      case "jobs":
        getItem = await contract.get_jobs_item({
          category: params.category,
          post_id: Number(params.id),
        });
        break;
      case "housing":
        getItem = await contract.get_housing_item({
          category: params.category,
          post_id: Number(params.id),
        });
        break;
      default:
        throw error(404, "Not Found");
    }

    return getItem;
  };

  return {
    group: params.group, 
    item: await getItemFromContract(),
  };
}
