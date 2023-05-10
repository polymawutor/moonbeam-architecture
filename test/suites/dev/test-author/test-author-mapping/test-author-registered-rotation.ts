import "@moonbeam-network/api-augment";
import { alith, BALTATHAR_SESSION_ADDRESS, CHARLETH_SESSION_ADDRESS } from "@moonwall/util";
import { getMappingInfo } from "../../../../helpers/common.js";
import { expect, describeSuite } from "@moonwall/cli";

describeSuite({
  id: "D227",
  title: "Author Mapping - registered can rotate",
  foundationMethods: "dev",
  testCases: ({ context, log, it }) => {
    it({
      id: "",
      title: "should succeed in rotating account ids for an author",
      test: async function () {
        await context.createBlock(
          context
            .polkadotJs({ type: "moon" })
            .tx.authorMapping.addAssociation(BALTATHAR_SESSION_ADDRESS)
        );
        expect((await getMappingInfo(context, BALTATHAR_SESSION_ADDRESS)).account).to.eq(
          alith.address
        );

        await context.createBlock(
          context
            .polkadotJs({ type: "moon" })
            .tx.authorMapping.updateAssociation(BALTATHAR_SESSION_ADDRESS, CHARLETH_SESSION_ADDRESS)
        );
        expect(await getMappingInfo(context, BALTATHAR_SESSION_ADDRESS)).to.eq(null);
        expect((await getMappingInfo(context, CHARLETH_SESSION_ADDRESS)).account).to.eq(
          alith.address
        );

        await context.createBlock();
      },
    });
  },
});
