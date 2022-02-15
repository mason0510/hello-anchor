const anchor = require("@project-serum/anchor");

describe("Hello Anchor!", () => {
  anchor.setProvider(anchor.Provider.local());

  it("Uses the workspace to invoke the initialize instruction", async () => {
    // #region code
    // Read the deployed program from the workspace.
    const program = anchor.workspace.HelloAnchor;
    console.log(program);
    // Execute the RPC.
    await program.rpc.initialize();
    // #endregion code
  });
});
