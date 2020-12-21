import styled from "styled-components";

export interface GridItem {
  id: number;
  poster: string | null;
  primary: string;
  secondary: string | null;
}

export interface Props {
  items: GridItem[];
  onItemClick?: (item: GridItem) => void;
}

export function Grid({ items, onItemClick }: Props) {
  return (
    <GridContainer>
      {items.map((item) => (
        <GridItemDiv key={item.id} onClick={() => onItemClick?.(item)}>
          <Poster src={item.poster} />
          <Details>
            <PrimaryText>{item.primary}</PrimaryText>
            <SecondaryText>{item.secondary}</SecondaryText>
          </Details>
        </GridItemDiv>
      ))}
    </GridContainer>
  );
}

const GridContainer = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  grid-gap: 8px;
`;

const GridItemDiv = styled.div`
  background: #0f0f0f;
  border-radius: 4px;
  overflow: hidden;
  user-select: none;
  cursor: pointer;
`;

const Poster = styled.div<{ src: string | null }>`
  background: ${({ src }) => (src ? `url(${src})` : "grey")};
  background-size: cover;
  background-position: center;
  padding-bottom: calc(100% * (278 / 185));
`;

const Details = styled.div`
  padding: 12px;
`;

const PrimaryText = styled.p`
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 10pt;
  font-weight: bold;
`;

const SecondaryText = styled.p`
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 10pt;
`;
