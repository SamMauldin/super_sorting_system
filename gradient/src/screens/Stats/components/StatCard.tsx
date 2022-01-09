import styled from "styled-components";

type Props = {
  title: string;
  value: string | number;
};

export const StatCard = ({ title, value }: Props) => (
  <Container>
    <Value>{value}</Value>
    <Title>{title}</Title>
  </Container>
);

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  background-color: ${({ theme }) => theme.blue};
  border: 3px solid black;
  border-radius: 20px;
  padding: 20px;

  min-width: 400px;
  min-height: 100px;
`;

const Value = styled.h1`
  margin: 10px 10px 0px 0px;
`;

const Title = styled.h2`
  margin: 10px 10px 0px 0px;
  font-size: 16px;
  font-weight: normal;
`;
