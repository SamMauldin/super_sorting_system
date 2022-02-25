import { KeyboardEvent, useEffect } from "react";
import styled from "styled-components";
import { useMutation, useQuery } from "react-query";
import { getSignConfig } from "../../api/automation";
import { SplashScreen } from "../SplashScreen";
import { pickupItems } from "../../api/pickup";
import { pathfindingNode } from "../../store";
import { useRecoilState } from "recoil";

/*
 *
 * Steps:
 * Select Items
 * Confirm Items + Delivery Destination
 *
 */

export const Pickup = () => {
  const { isLoading, isError, data } = useQuery("sign_config", getSignConfig);

  const [pickupLoc, setPickupLoc] = useRecoilState(pathfindingNode);

  useEffect(() => {
    if (!data || pickupLoc === null) return;

    if (!data.data.nodes[pickupLoc]) {
      setPickupLoc(null);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pickupLoc, data]);

  const { mutate, status, error, reset } = useMutation(
    "pickup",
    async ({ loc }: { loc: string }) => await pickupItems(loc)
  );

  if (status === "success")
    return (
      <SplashScreen
        message={
          <>
            <p>Pickup successful!</p>
            <button onClick={reset} autoFocus>
              Back
            </button>
          </>
        }
      />
    );
  if (status === "error")
    return (
      <SplashScreen
        message={
          <>
            <p>Pickup failed!</p>
            <p>{String(error)}</p>
            <button onClick={reset} autoFocus>
              Back
            </button>
          </>
        }
      />
    );
  if (status === "loading")
    return <SplashScreen message="Pickup in progress" />;

  if (isLoading) return <SplashScreen message="Loading pickup location data" />;
  if (isError || !data)
    return <SplashScreen message="Failed to load pickup location data" />;

  const pickup = () => mutate({ loc: pickupLoc! });

  const onKeyDown = (ev: KeyboardEvent) => {
    if (ev.key === "Enter" && pickupLoc) pickup();
  };

  return (
    <Container>
      <p>Pickup from:</p>
      <select
        value={pickupLoc || ""}
        onChange={({ target: { value } }) => setPickupLoc(value)}
        onKeyDown={onKeyDown}
        autoFocus
      >
        <option value="">-- Please select a pickup location --</option>
        {Object.values(data.data.nodes)
          .filter((node) => Boolean(node.pickup))
          .map((node) => (
            <option key={node.name} value={node.name}>
              {node.name}
            </option>
          ))}
      </select>
      <button disabled={pickupLoc === null} onClick={pickup}>
        Pickup
      </button>
    </Container>
  );
};

const Container = styled.div`
  margin: 1em;
  display: flex;
  flex-direction: column;

  * {
    margin-top: 0px;
    margin-bottom: 1em;
  }
`;
