import React, { FC } from "react";
import { Image, StyleSheet, View } from "react-native";
import { StatusBar } from "expo-status-bar";
import { useRoute } from "@react-navigation/core";
import { Text, useTheme } from "react-native-paper";

import { TvShow } from "../api";

export const ShowDetailsScreen: FC = () => {
  const theme = useTheme();
  const { params } = useRoute();
  const show = (params as any).show as TvShow;
  return (
    <View style={styles.container}>
      <StatusBar translucent />
      {show.backdrop && (
        <Image source={{ uri: show.backdrop }} style={styles.backdrop} />
      )}
      <View style={styles.headerRow}>
        <Image
          source={{ uri: show.poster!! }}
          style={{
            ...styles.poster,
            borderRadius: theme.roundness,
          }}
        />
        <View style={styles.headerRowTextContainer}>
          <Text style={styles.showName}>{show.name}</Text>
          <Text style={styles.showYear}>{show.startYear()}</Text>
        </View>
      </View>
      <View style={styles.overviewContainer}>
        <Text>{show.overview}</Text>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  backdrop: {
    width: "100%",
    aspectRatio: 16 / 9,
  },

  headerRow: {
    marginTop: -50,
    padding: 16,
    flexDirection: "row",
    alignItems: "center",
  },

  poster: {
    width: 150,
    aspectRatio: 2 / 3,
  },

  headerRowTextContainer: {
    marginLeft: 16,
  },

  showName: {
    fontSize: 24,
  },

  showYear: {
    fontSize: 13,
  },

  overviewContainer: {
    marginHorizontal: 16,
  },
});
