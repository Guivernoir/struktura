/**
 * @file hooks/contractor/useCatalogue.js
 * @description Catalogue loading and category management
 * Mission objective: Intelligence gathering for available calculators
 */

import { useState, useEffect, useMemo } from "react";
import { api } from "../../lib";
import { DEFAULT_CATEGORY } from "./types";

export function useCatalogue(onCalculatorAutoSelect) {
  const [catalogue, setCatalogue] = useState(null);
  const [selectedCategory, setSelectedCategory] = useState(DEFAULT_CATEGORY);
  const [error, setError] = useState(null);

  // Load catalogue on mount - initial reconnaissance
  useEffect(() => {
    const load = async () => {
      try {
        const data = await api.calculus.contractor.getCatalogue();
        setCatalogue(data);

        // Auto-select first calculator in default category
        const first = data.calculators.find(
          (c) => c.category === selectedCategory
        );
        if (first && onCalculatorAutoSelect) {
          onCalculatorAutoSelect(first.id);
        }
      } catch (err) {
        setError("Failed to load contractor calculators");
        console.error("Contractor catalogue load failed:", err);
      }
    };
    load();
  }, []);

  // Memoize calculators in selected category - efficient reconnaissance
  const calculatorsInCategory = useMemo(() => {
    if (!catalogue) return [];
    return catalogue.calculators
      .filter((c) => c.category === selectedCategory)
      .map((c) => ({
        value: c.id,
        label: c.name,
        metadata: c,
      }));
  }, [catalogue, selectedCategory]);

  // Get categories list for navigation
  const categories = useMemo(() => {
    if (!catalogue) return [];
    return catalogue.categories || [];
  }, [catalogue]);

  return {
    catalogue,
    categories,
    selectedCategory,
    setSelectedCategory,
    calculatorsInCategory,
    error,
  };
}
