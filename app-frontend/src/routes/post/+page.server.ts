import type { Actions } from "./$types";
import { z } from "zod";
//   creator: wallet,
//  post_id: Number(Math.floor(Math.random() * 3654651989125135)),
//  date: Date.now(),

export const actions: Actions = {
  default: async ({ request }) => {
    console.log(request);

    const formData = Object.fromEntries(await request.formData());

    console.log(formData);

    const itemInfo = z.object({
      category: z.string().min(1).trim(),
      title: z.string().min(1).max(255).trim(),
      description: z.string().min(25).trim(),
      image: z.nullable(z.string()).optional(), // for now might have time to update
      location: z.nullable(z.string().min(1).max(255).trim().optional()),
      price: z.nullable(z.number().max(20).optional()),
      postal_code: z.nullable(z.number().max(5).optional()),
      city_or_neighborhood: z.nullable(
        z.string().min(1).max(100).trim().optional()
      ),
    });

    itemInfo.passthrough().parse({
      formData,
      creator: wallet.getAccountId(),
      post_id: Number(Math.floor(Math.random() * 3654651989125135)),
      date: Date.now(),
    });

    // const firstChar = formData.group.charAt(0).toLowerCase();

    // if (firstChar === "j" ){

    // } else if (firstChar === "h") {

    // } else if (firstChar === "f" || firstChar === "w") {

    // } else if (firstChar === "c") {

    // } else {
    //   return console.log("This did not work");

    // }
  },
};
