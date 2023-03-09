import styled from 'styled-components';

export const Help = () => (
  <Container>
    <h3>Keyboard Navigation</h3>
    <h4>Page Navigation</h4>
    <ul>
      <li>Ctrl + D: Open Delivery page</li>
      <li>Ctrl + P: Open Pickup page</li>
      <li>Ctrl + C: Open Config page</li>
      <li>Ctrl + S: Open Stats page</li>
      <li>Ctrl + H: Open Help page</li>
    </ul>
    <h4>Item Selection</h4>
    <ul>
      <li>Typing: Search available items</li>
      <li>Up / Down: Move item cursor up or down</li>
      <li>Enter: Select an item</li>
      <li>
        Shift + Enter: Confirm existing selections and move to next screen
      </li>
    </ul>
    <h4>Item Count Selection</h4>
    <ul>
      <li>Up / Down: Change the number of items you want selected</li>
      <li>
        Tab: Change the increment you are selecting from item, stack, shulker
      </li>
      <li>Enter: Set the selection count and close the modal</li>
    </ul>
    <h4>Destination Selection</h4>
    <ul>
      <li>Typing: Search destinations</li>
      <li>Up / Down: Switch selected destination</li>
      <li>Enter: Confirm selection</li>
    </ul>
  </Container>
);

const Container = styled.div`
  margin: 10px;
`;
