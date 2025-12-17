import { createBrowserRouter, Navigate } from "react-router-dom";
import { api } from "./lib";

// Layouts
import RootLayout from "./layouts/RootLayout";
import DashboardLayout from "./layouts/DashboardLayout";

// Pages
import Home from "./pages/Home";
import Auth from "./pages/Auth";
import Guide from "./pages/Guide";
import BeginnerCalculator from "./pages/BeginnerCalculator";
import ContractorCalculator from "./pages/ContractorCalculator";
import EngineerCalculator from "./pages/EngineerCalculator";
import Dashboard from "./pages/Dashboard";
import Projects from "./pages/Projects";
import History from "./pages/History";
import Profile from "./pages/Profile";
import NotFound from "./pages/NotFound";

// =============================================================================
// PROTECTED ROUTE COMPONENT
// =============================================================================

const ProtectedRoute = ({ children }) => {
  if (!api.auth.isAuthenticated()) {
    return <Navigate to="/auth" replace />;
  }
  return children;
};

// =============================================================================
// ROUTER CONFIGURATION
// =============================================================================

export const router = createBrowserRouter([
  {
    path: "/",
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "auth",
        element: <Auth />,
      },
      {
        path: "guide",
        element: <Guide />,
      },
      {
        path: "beginner",
        element: <BeginnerCalculator />,
      },
      {
        path: "contractor",
        element: <ContractorCalculator />,
      },
      {
        path: "engineer",
        element: <EngineerCalculator />,
      },
    ],
  },
  {
    path: "/dashboard",
    element: (
      <ProtectedRoute>
        <DashboardLayout />
      </ProtectedRoute>
    ),
    children: [
      {
        index: true,
        element: <Dashboard />,
      },
      {
        path: "projects",
        element: <Projects />,
      },
      {
        path: "history",
        element: <History />,
      },
      {
        path: "profile",
        element: <Profile />,
      },
    ],
  },
  {
    path: "*",
    element: <NotFound />,
  },
]);

export default router;
