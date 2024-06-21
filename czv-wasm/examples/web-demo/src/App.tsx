import "./App.css";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import init, * as czv from "czv-wasm";
import React, { useState } from "react";

function App() {
    const [initDone, setInitDone] = useState(false);
    const [loading, setLoading] = useState(false);
    const [rowCount, setRowCount] = useState<number | undefined>(undefined);
    const [columnCount, setColumnCount] = useState<number | undefined>(
        undefined
    );

    const handleFile = async (e: React.ChangeEvent<HTMLInputElement>) => {
        setLoading(true);
        if (!initDone) {
            await init();
            setInitDone(true);
        }
        if (e.target.files && e.target.files.length === 1) {
            const rowCountOutput = czv.rowCount({
                file_data: await e.target.files[0].text(),
            });
            setRowCount(rowCountOutput);
            const columnCountOutput = czv.columnCount({
                file_data: await e.target.files[0].text(),
            });
            setColumnCount(columnCountOutput);
        }
        setLoading(false);
    };

    return (
        <>
            <div className="grid text-left w-full gap-1.5">
                <hgroup>
                    <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
                        czv - WebAssembly library demo using Vite & React
                    </h1>
                    <p className="text-xl text-muted-foreground mb-4">
                        Visit{" "}
                        <a
                            className="text-cyan-400"
                            href="https://github.com/rzmk/czv"
                        >
                            here
                        </a>{" "}
                        for the czv source code and{" "}
                        <a
                            className="text-cyan-400"
                            href="https://github.com/rzmk/czv/tree/main/czv-wasm/examples/web-demo"
                        >
                            here
                        </a>{" "}
                        for this site's source code.
                    </p>
                </hgroup>
                <Label htmlFor="csv">Import your CSV file</Label>
                <Input onChange={handleFile} id="csv" type="file" />

                {loading && <p>Loading...</p>}

                {rowCount && (
                    <p>
                        <strong>Row count (excluding header row)</strong>:{" "}
                        {rowCount}
                    </p>
                )}
                {columnCount && (
                    <p>
                        <strong>Column count</strong>: {columnCount}
                    </p>
                )}
            </div>
        </>
    );
}

export default App;
