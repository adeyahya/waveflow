import React from "react";
import { ChakraProvider } from "@chakra-ui/react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { useMount } from "react-use";

import Root from "~pages";
import Login from "~pages/login";
import NotFound from "~pages/not-found";
import NewWorkflow from "~pages/workflows/new";
import useUserStore from "~store/user";

const App = () => {
  const user = useUserStore();

  useMount(() => {
    user.fetch();
  });

  if (user.loading) return null;

  return (
    <ChakraProvider>
      <React.StrictMode>
        <Router>
          <Switch>
            <Route exact path="/">
              <Root />
            </Route>
            <Route path="/login">
              <Login />
            </Route>
            <Route path="/workflow/new">
              <NewWorkflow />
            </Route>
            <Route path="*">
              <NotFound />
            </Route>
          </Switch>
        </Router>
      </React.StrictMode>
    </ChakraProvider>
  );
};

export default App;
