import { useQuery } from "react-query";
import styled from "styled-components";
import { getStats } from "../../api/admin";
import { SplashScreen } from "../SplashScreen";
import { StatCard } from "./components";

export const Stats = () => {
  const { error, isLoading, data } = useQuery("stats", getStats, {
    refetchInterval: 1000 * 3,
  });

  if (error) return <SplashScreen message="Error loading stats!" />;
  if (isLoading || !data) return <SplashScreen message="Loading stats" />;

  const { data: stats } = data;

  return (
    <Container>
      <RowTitle>Operations</RowTitle>
      <Row>
        <StatCard title="Operations Pending" value={stats.operations_pending} />
        <StatCard
          title="Operations In Progress"
          value={stats.operations_in_progress}
        />
      </Row>
      <RowTitle>Storage</RowTitle>
      <Row>
        <StatCard title="Inventories Loaded" value={stats.inventories_in_mem} />
        <StatCard title="Slots Free" value={stats.free_slots} />
        <StatCard title="Total Slots" value={stats.total_slots} />
      </Row>
      <RowTitle>Internals</RowTitle>
      <Row>
        <StatCard title="Agents Connected" value={stats.agents_connected} />
        <StatCard title="Slot Holds" value={stats.current_holds} />
      </Row>
    </Container>
  );
};

const Container = styled.div`
  margin: 10px;
`;

const RowTitle = styled.h1`
  text-align: center;
`;

const Row = styled.div`
  display: flex;
  justify-content: center;

  & > * {
    margin-right: 10px;
  }
`;
