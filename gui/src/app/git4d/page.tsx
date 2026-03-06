import { connection } from "next/server";
import Git4DPageClient from "./Git4DPageClient";

export default async function Git4DPage() {
  await connection();
  return <Git4DPageClient />;
}
