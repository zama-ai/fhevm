import config from "../config";
import { useQuery } from "@tanstack/react-query";

export default function usePlans() {
  const {data, error, isLoading} = useQuery({ queryKey: ['plans'],
    queryFn: async () => {
      const response = await fetch(`${config.devPortalApiServer}/plans`);
      if (!response.ok) {
        console.error(`Failed to fetch plans. Status: ${response.status} ${response.statusText}`);
        throw new Error(`Failed to fetch plans: ${response.status} ${response.statusText}`);
      }
      const result = await response.json();
      return (result?.hits || []).filter(function(item) {
        return item.status === 'active'
      })
    }
  })

  return { 
    plans: data,
    plansLoading: isLoading,
    plansError: error,
  }
}

