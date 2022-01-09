import { KeyboardEvent, useEffect, useState } from "react";
import { ItemSelector } from "../../common";
import { ExtendedItem } from "../../helpers";
import styled from "styled-components";
import { useMutation, useQuery } from "react-query";
import { getPathfindingConfig } from "../../api/automation";
import { SplashScreen } from "../SplashScreen";
import { deliverItems } from "../../api/delivery";
import { useRecoilState } from "recoil";
import { pathfindingNode } from "../../store";

/*
 *
 * Steps:
 * Select Items
 * Confirm Items + Delivery Destination
 *
 */

export const Delivery = () => {
  const [selectedItems, setSelectedItems] = useState<
    | {
        item: ExtendedItem;
        count: number;
      }[]
    | null
  >(null);

  const { isLoading, isError, data } = useQuery(
    "pathfinding_config",
    getPathfindingConfig
  );

  const [deliveryLoc, setDeliveryLoc] = useRecoilState(pathfindingNode);

  useEffect(() => {
    if (!data || deliveryLoc === null) return;

    if (!data.data.nodes[deliveryLoc]) {
      setDeliveryLoc(null);
    }
  }, [deliveryLoc, data]);

  const { mutate, status, error, reset } = useMutation(
    "delivery",
    async ({
      loc,
      items,
    }: {
      loc: string;
      items: {
        item: ExtendedItem;
        count: number;
      }[];
    }) => await deliverItems(loc, items)
  );

  const back = () => {
    reset();
    setSelectedItems(null);
  };

  if (status === "success")
    return (
      <SplashScreen
        message={
          <>
            <p>Delivery successful!</p>
            <button onClick={back}>Back</button>
          </>
        }
      />
    );
  if (status === "error")
    return (
      <SplashScreen
        message={
          <>
            <p>Delivery failed!</p>
            <p>{String(error)}</p>
            <button onClick={back}>Back</button>
          </>
        }
      />
    );
  if (status === "loading")
    return <SplashScreen message="Delivery in progress" />;

  if (!selectedItems)
    return <ItemSelector submit={(selected) => setSelectedItems(selected)} />;

  if (isLoading)
    return <SplashScreen message="Loading delivery location data" />;
  if (isError || !data)
    return <SplashScreen message="Failed to load delivery location data" />;

  const deliver = () => mutate({ loc: deliveryLoc!, items: selectedItems });

  const onKeyDown = (ev: KeyboardEvent) => {
    if (ev.key === "Enter" && deliveryLoc) deliver();
  };

  return (
    <Container>
      {selectedItems.length === 0 ? (
        <>
          <p>You did not select any items!</p>
        </>
      ) : (
        <>
          <p>Selected items:</p>
          <ul>
            {selectedItems.map((item) => (
              <li key={item.item.stackable_hash}>
                {item.item.prettyPrinted} x{item.count}
              </li>
            ))}
          </ul>
          <p>Deliver to:</p>
          <select
            value={deliveryLoc || ""}
            onChange={({ target: { value } }) => setDeliveryLoc(value)}
            onKeyDown={onKeyDown}
            autoFocus
          >
            <option value="">-- Please select a delivery location --</option>
            {Object.entries(data.data.nodes)
              .filter(([_node_id, node]) => Boolean(node.portal))
              .map(([node_id, node]) => (
                <option key={node_id} value={node_id}>
                  {node.pretty_name || node_id}
                </option>
              ))}
          </select>
          <button disabled={deliveryLoc === null} onClick={deliver}>
            Deliver
          </button>
        </>
      )}
      <button onClick={() => setSelectedItems(null)}>Back</button>
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
