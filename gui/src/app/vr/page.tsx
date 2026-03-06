import { connection } from "next/server";
import VRPageClient from "./VRPageClient";

export default async function VRPage() {
  await connection();
  return <VRPageClient />;
}
