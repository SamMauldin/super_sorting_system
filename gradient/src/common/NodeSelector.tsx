import { useEffect, useRef, useState } from 'react';
import { useQuery } from 'react-query';
import { getSignConfig } from '../api/automation';
import styled, { css } from 'styled-components';
import { Fzf } from 'fzf';

type Props = {
  submit: (node: string) => void;
  purpose: 'delivery' | 'pickup';
};

const getLastUsedNodes = (): string[] => {
  try {
    return JSON.parse(localStorage['used_node_list']);
  } catch (_) {
    return [];
  }
};

const markNodeUsed = (node: string) => {
  let nodes = getLastUsedNodes();
  nodes = nodes.filter((other_node) => other_node !== node);
  nodes.unshift(node);

  while (nodes.length > 50) {
    nodes.pop();
  }

  localStorage['used_node_list'] = JSON.stringify(nodes);
};

const negativeOneToInfinity = (num: number) => {
  if (num === -1) return Infinity;
  return num;
};

const nodeSortFactory = (priorityList: string[]) => (a: string, b: string) => {
  let aLastUsed = negativeOneToInfinity(priorityList.indexOf(a));
  let bLastUsed = negativeOneToInfinity(priorityList.indexOf(b));

  if (aLastUsed > bLastUsed) {
    return 1;
  } else if (aLastUsed < bLastUsed) {
    return -1;
  }

  return b.localeCompare(a);
};

export const NodeSelector = ({ submit: rawSubmit, purpose }: Props) => {
  const [hoverIdx, setHoverIdx] = useState(0);
  const [search, setSearch] = useState('');
  const mainInputRef = useRef<HTMLInputElement>(null);
  const priorityList = getLastUsedNodes();
  const nodeSort = nodeSortFactory(priorityList);

  const submit = (node: string) => {
    markNodeUsed(node);
    rawSubmit(node);
  };

  const { isLoading, isError, data } = useQuery('sign_config', getSignConfig);

  const dataNodes = data?.data?.nodes ? Object.values(data.data.nodes) : [];
  const validNodes = dataNodes
    .filter((node) => {
      if (purpose === 'delivery') return node.dropoff;
      if (purpose === 'pickup') return node.pickup;
      return false;
    })
    .map((node) => node.name);

  const fzf = new Fzf(validNodes, {
    tiebreakers: [(a, b) => nodeSort(a.item, b.item)],
  });

  const nodes =
    search.length > 0
      ? fzf.find(search).map((res) => res.item)
      : validNodes.sort(nodeSort);

  const clampHoverIdx = (idx: number) =>
    Math.max(0, Math.min(idx, nodes.length - 1));

  useEffect(() => {
    setHoverIdx(clampHoverIdx(hoverIdx));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [nodes.length]);

  if (isLoading) return <p>Loading pathfinding nodes</p>;
  if (isError) return <p>Failed to load pathfinding nodes</p>;

  const onKeyDown = (ev: React.KeyboardEvent<HTMLInputElement>) => {
    if (ev.key === 'ArrowUp' || ev.key === 'ArrowDown') {
      const up = ev.key === 'ArrowUp';

      setHoverIdx(clampHoverIdx(hoverIdx + (up ? -1 : 1)));

      ev.preventDefault();
    }

    if (ev.key === 'Enter') {
      const node = nodes[hoverIdx];

      submit(node);
    }
  };

  return (
    <Container>
      <UpperContainer>
        <NodeInput
          autoFocus={true}
          ref={mainInputRef}
          type="text"
          onKeyDown={onKeyDown}
          value={search}
          onChange={({ target: { value } }) => setSearch(value || '')}
        />
      </UpperContainer>
      <InnerContainer>
        <NodeList>
          {nodes.map((node, idx) => {
            return (
              <NodeOption
                key={idx}
                hovered={hoverIdx === idx}
                ref={
                  hoverIdx === idx
                    ? (elem) => elem?.scrollIntoView({ block: 'nearest' })
                    : undefined
                }
                onClick={() => submit(node)}
              >
                {node}
              </NodeOption>
            );
          })}
        </NodeList>
      </InnerContainer>
    </Container>
  );
};

const Container = styled.div`
  display: grid;
  grid-template-rows: 25px 1fr;
  grid-template-columns: 1fr;
  max-height: 400px;
`;

const InnerContainer = styled.div`
  overflow-y: auto;
`;

const NodeInput = styled.input`
  flex-grow: 1;
  outline: none;
`;

const UpperContainer = styled.div`
  display: flex;
`;

const NodeList = styled.ul`
  margin: 0;
  padding: 0;
  list-style: none;
`;

const NodeOption = styled.li<{ hovered: boolean }>`
  padding: 4px;

  ${({ hovered }) =>
    hovered
      ? css`
          background-color: ${({ theme }) => theme.fg0};
          color: ${({ theme }) => theme.bg0};
        `
      : ''};
`;
