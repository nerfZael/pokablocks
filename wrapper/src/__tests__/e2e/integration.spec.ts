import { PolywrapClient } from "@polywrap/client-js";
import path from "path";

jest.setTimeout(60000);

describe("Template Wrapper End to End Tests", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("metadata method", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "metadata",
      args: {
        id: "2"
      }
    });

    console.log(JSON.parse(result.data as string));
    // expect(error).toBeFalsy();
    // expect(data?.value).toEqual(expected);
  });

  it("animation method", async () => {
    const result = await client.invoke({
      uri: wrapperUri,
      method: "animation",
      args: {
        id: "2"
      }
    });

    console.log(result);
    // expect(error).toBeFalsy();
    // expect(data?.value).toEqual(expected);
  });
});
