import React from "react";
import { BrowserRouter, Switch, Route } from "react-router-dom";
import { Page } from "./types";
import ScrollToTop from "./ScrollToTop";
import Navbar from "../components/Navbar";
import Page404 from "../pages/Page404";
import PageHome from "../pages/PageHome";

export const pages: Page[] = [
  { path: "/", exact: true, component: PageHome },
  { path: "/#", exact: true, component: PageHome },
  //
];

const Routes = () => {
  return (
    <BrowserRouter>
      <ScrollToTop />
      <Navbar />
      <Switch>
        {pages.map(({ component, path, exact }) => {
          return (
            <Route
              key={path}
              component={component}
              exact={!!exact}
              path={path}
            />
          );
        })}
        <Route component={Page404} />
      </Switch>
    </BrowserRouter>
  );
};

export default Routes;
