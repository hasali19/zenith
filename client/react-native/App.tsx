import React, { FC } from "react";
import { Text, useColorScheme, View } from "react-native";
import { StatusBar } from "expo-status-bar";
import { NavigationContainer, useTheme } from "@react-navigation/native";
import { createStackNavigator } from "@react-navigation/stack";
import { navigationTheme } from "./theme";

const HomeScreen: FC = () => {
  const theme = useTheme();
  return (
    <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
      <Text style={{ color: theme.colors.text }}>Home Screen</Text>
    </View>
  );
};

const Stack = createStackNavigator();

const App: FC = () => {
  const scheme = useColorScheme();
  const theme = navigationTheme(scheme);
  return (
    <NavigationContainer theme={theme}>
      <StatusBar />
      <Stack.Navigator>
        <Stack.Screen name="Home" component={HomeScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  );
};

export default App;
