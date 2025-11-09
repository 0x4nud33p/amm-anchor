import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AmmAnchor } from "../target/types/amm_anchor";

describe("amm-anchor", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AmmAnchor as Program<AmmAnchor>;

  

  before("", async () => {

  });

  it("initializing the amm pool", async () => {
      
    }),

});
