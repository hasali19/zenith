import React, { FC, useEffect, useState } from "react";
import {
  FlatList,
  Image,
  StyleSheet,
  useWindowDimensions,
  View,
} from "react-native";
import { Text, useTheme } from "react-native-paper";

import api, { TvShow } from "../api";

export const ShowsScreen: FC = () => {
  const { width } = useWindowDimensions();
  const [shows, setShows] = useState<TvShow[] | null>(null);

  useEffect(() => {
    api.tv.getShows().then(setShows);
  }, []);

  const padding = 4;
  const columnCount = Math.min(Math.floor((width - padding * 2) / 130), 10);
  const columnWidth = (width - padding * 2) / columnCount;

  return (
    <FlatList
      key={columnCount}
      numColumns={columnCount}
      data={shows || []}
      contentContainerStyle={{ padding }}
      keyExtractor={(item) => item.id.toString()}
      renderItem={({ item }) => (
        <GridItem item={item} width={columnWidth} padding={padding} />
      )}
    />
  );
};

const GridItem: FC<{ item: TvShow; width: number; padding: number }> = ({
  item,
  width,
  padding,
}) => {
  const theme = useTheme();

  const posterWidth = width - padding * 2;
  const posterHeight = posterWidth * (3 / 2);

  return (
    <View style={{ width, padding }}>
      {item.poster ? (
        <Image
          source={{ uri: item.poster }}
          style={{
            width: posterWidth,
            height: posterHeight,
            borderRadius: theme.roundness,
          }}
        />
      ) : (
        <View
          style={{
            width: posterWidth,
            height: posterHeight,
            borderRadius: theme.roundness,
            backgroundColor: "rgb(50, 50, 50)",
          }}
        />
      )}
      <Text numberOfLines={1} style={styles.itemName}>
        {item.name}
      </Text>
      <Text numberOfLines={1} style={styles.itemYear}>
        {item.startYear()}
      </Text>
    </View>
  );
};

const styles = StyleSheet.create({
  itemName: {
    marginTop: 8,
    fontSize: 13,
    fontWeight: "bold",
  },

  itemYear: {
    marginBottom: 8,
    fontSize: 11,
  },
});
