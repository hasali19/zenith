import React, { FC } from "react";
import { useColorScheme, View } from "react-native";
import { StatusBar } from "expo-status-bar";
import { NavigationContainer } from "@react-navigation/native";
import { createStackNavigator } from "@react-navigation/stack";
import { Button, Provider as PaperProvider, Text } from "react-native-paper";
import { navigationTheme } from "./theme";

const HomeScreen: FC = () => (
  <View style={{ flex: 1, alignItems: "center", justifyContent: "center" }}>
    <Text>Home Screen</Text>
    <Button
      mode="contained"
      style={{ margin: 8 }}
      onPress={() => console.log("Hello, world!")}
    >
      Click Me
    </Button>
  </View>
);

const Stack = createStackNavigator();

const App: FC = () => {
  const scheme = useColorScheme();
  const theme = navigationTheme(scheme);
  return (
    <PaperProvider>
      <NavigationContainer theme={theme}>
        <StatusBar />
        <Stack.Navigator>
          <Stack.Screen name="Home" component={HomeScreen} />
        </Stack.Navigator>
      </NavigationContainer>
    </PaperProvider>
  );
};

export default App;
