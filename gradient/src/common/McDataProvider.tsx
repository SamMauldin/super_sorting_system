import React, { useContext, createContext, useMemo } from "react";
import { useQuery } from "react-query";
import { getEnchantments, getItems } from "../api/data";
import { EnchantmentTypeByName, ItemTypeById } from "../api/data_types";
import { SplashScreen } from "../screens/SplashScreen";

export type McData = {
  items: ItemTypeById;
  enchantments: EnchantmentTypeByName;
};

export const McDataContext = createContext<McData>(null as any);

export const McDataProvider: React.FC<{
  children: React.ReactChild;
}> = ({ children }) => {
  const {
    isLoading: mcItemsLoading,
    isError: mcItemsError,
    data: mcItemsArr,
  } = useQuery("mc_data_items", getItems, {
    staleTime: Infinity,
    cacheTime: Infinity,
  });

  const {
    isLoading: mcEnchantmentsLoading,
    isError: mcEnchantmentsError,
    data: mcEnchantmentsArr,
  } = useQuery("mc_data_enchants", getEnchantments, {
    staleTime: Infinity,
    cacheTime: Infinity,
  });

  const items = useMemo(() => {
    if (!mcItemsArr) return undefined;

    const map: ItemTypeById = new Map();

    for (let item of mcItemsArr.data) {
      map.set(item.id, item);
    }

    return map;
  }, [mcItemsArr]);

  const enchantments = useMemo(() => {
    if (!mcEnchantmentsArr) return undefined;

    const map: EnchantmentTypeByName = new Map();

    for (let enchant of mcEnchantmentsArr.data) {
      map.set(enchant.name, enchant);
    }

    return map;
  }, [mcEnchantmentsArr]);

  if (mcItemsLoading || mcEnchantmentsLoading)
    return <SplashScreen message="Loading Minecraft Data" />;

  if (mcItemsError || mcEnchantmentsError)
    return <SplashScreen message="Failed to load Minecraft Data!" />;

  return (
    <McDataContext.Provider
      value={{ items: items!, enchantments: enchantments! }}
    >
      {children}
    </McDataContext.Provider>
  );
};

export const useMcData = (): McData => useContext(McDataContext);
