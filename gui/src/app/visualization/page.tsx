import { connection } from "next/server";
import VisualizationPageClient from "./VisualizationPageClient";

export default async function VisualizationPage() {
  await connection();
  return <VisualizationPageClient />;
}
