import React, { FC } from "react";
import { useColorScheme, View } from "react-native";
import { StatusBar } from "expo-status-bar";
import { NavigationContainer } from "@react-navigation/native";
import { createStackNavigator } from "@react-navigation/stack";
import { Provider as PaperProvider } from "react-native-paper";

import { navigationTheme } from "./theme";
import { ShowsScreen } from "./screens/Shows";
import { ShowDetailsScreen } from "./screens/ShowDetails";

const Stack = createStackNavigator();

const App: FC = () => {
  const scheme = useColorScheme();
  const theme = navigationTheme(scheme);
  return (
    <View style={{ flex: 1, backgroundColor: theme.colors.background }}>
      <PaperProvider>
        <NavigationContainer theme={theme}>
          <StatusBar />
          <Stack.Navigator>
            <Stack.Screen
              name="Shows"
              component={ShowsScreen}
              options={{ headerTitle: "Zenith" }}
            />
            <Stack.Screen
              name="ShowDetails"
              component={ShowDetailsScreen}
              options={{ headerShown: false }}
            />
          </Stack.Navigator>
        </NavigationContainer>
      </PaperProvider>
    </View>
  );
};

export default App;
