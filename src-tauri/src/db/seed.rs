use crate::db::Database;
use sqlx::Error;

pub async fn seed_tools(db: &Database) -> Result<(), Error> {
    // Check if tools exist to avoid global duplicate seeding, BUT force TIU check
    let tools = db.get_all_tools().await.unwrap_or_default();
    let tiu_exists = tools.iter().any(|t| t.name == "TIU");

    // Force TIU Content Check
    if !tiu_exists {
         seed_tiu(db).await?;
    } else {
         println!("TIU already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("TIU").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("TIU is empty. Re-seeding TIU content...");
                 seed_tiu_content(db, tool.id).await?;
             }
         }
    }

    // Force IST Content Check
    let ist_exists = tools.iter().any(|t| t.name == "IST");
    if !ist_exists {
        seed_ist(db).await?;
    } else {
         println!("IST already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("IST").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("IST is empty. Re-seeding IST content...");
                 seed_ist_content(db, tool.id).await?;
             }
         }
    }

    // Force CFIT Content Check
    let cfit_exists = tools.iter().any(|t| t.name == "CFIT");
    if !cfit_exists {
        seed_cfit(db).await?;
    } else {
         println!("CFIT already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("CFIT").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("CFIT is empty. Re-seeding CFIT content...");
                 seed_cfit_content(db, tool.id).await?;
             }
         }
    }

    // Force Kraepelin Content Check
    let kraepelin_exists = tools.iter().any(|t| t.name == "KRAEPELIN");
    if !kraepelin_exists {
        seed_kraepelin(db).await?;
    } else {
         println!("KRAEPELIN already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("KRAEPELIN").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("KRAEPELIN is empty. Re-seeding KRAEPELIN content...");
                 seed_kraepelin_content(db, tool.id).await?;
             }
         }
    }

    // Force MATRICES Content Check
    let matrices_exists = tools.iter().any(|t| t.name == "MATRICES");
    if !matrices_exists {
        seed_matrices(db).await?;
    } else {
         println!("MATRICES already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("MATRICES").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("MATRICES is empty. Re-seeding MATRICES content...");
                 seed_matrices_content(db, tool.id).await?;
             }
         }
    }

    // Force WPT Content Check
    let wpt_exists = tools.iter().any(|t| t.name == "WPT");
    if !wpt_exists {
        seed_wpt(db).await?;
    } else {
         println!("WPT already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("WPT").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("WPT is empty. Re-seeding WPT content...");
                 seed_wpt_content(db, tool.id).await?;
             }
         }
    }

    // Force DISC Content Check
    let disc_exists = tools.iter().any(|t| t.name == "DISC");
    if !disc_exists {
        seed_disc(db).await?;
    } else {
         println!("DISC already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("DISC").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("DISC is empty. Re-seeding DISC content...");
                 seed_disc_content(db, tool.id).await?;
             }
         }
    }

    // Force MBTI Content Check
    let mbti_exists = tools.iter().any(|t| t.name == "MBTI");
    if !mbti_exists {
        seed_mbti(db).await?;
    } else {
         println!("MBTI already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("MBTI").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("MBTI is empty. Re-seeding MBTI content...");
                 seed_mbti_content(db, tool.id).await?;
             }
         }
    }

    // Force GATB Content Check
    let gatb_exists = tools.iter().any(|t| t.name == "GATB");
    if !gatb_exists {
        seed_gatb(db).await?;
    } else {
         println!("GATB already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("GATB").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("GATB is empty. Re-seeding GATB content...");
                 seed_gatb_content(db, tool.id).await?;
             }
         }
    }

    // Force EPPS Content Check
    let epps_exists = tools.iter().any(|t| t.name == "EPPS");
    if !epps_exists {
        seed_epps(db).await?;
    } else {
         println!("EPPS already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("EPPS").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("EPPS is empty. Re-seeding EPPS content...");
                 seed_epps_content(db, tool.id).await?;
             }
         }
    }

    // Force PAPI Content Check
    let papi_exists = tools.iter().any(|t| t.name == "PAPI");
    if !papi_exists {
        seed_papi(db).await?;
    } else {
         println!("PAPI already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("PAPI").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("PAPI is empty. Re-seeding PAPI content...");
                 seed_papi_content(db, tool.id).await?;
             }
         }
    }

    // Force 16PF Content Check
    let pf16_exists = tools.iter().any(|t| t.name == "16PF");
    if !pf16_exists {
        seed_16pf(db).await?;
    } else {
         println!("16PF already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("16PF").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("16PF is empty. Re-seeding 16PF content...");
                 seed_16pf_content(db, tool.id).await?;
             }
         }
    }

    // Force HEXACO Content Check
    let hexaco_exists = tools.iter().any(|t| t.name == "HEXACO");
    if !hexaco_exists {
        seed_hexaco(db).await?;
    } else {
         println!("HEXACO already exists. Checking content...");
         if let Ok(tool) = db.get_tool_by_name("HEXACO").await {
             let subtests = db.get_subtests_by_tool(tool.id).await.unwrap_or_default();
             if subtests.is_empty() {
                 println!("HEXACO is empty. Re-seeding HEXACO content...");
                 seed_hexaco_content(db, tool.id).await?;
             }
         }
    }

    // Only seed other tools if table was completely empty (first run)
    if tools.is_empty() {
        // [A] Cognitive & Intelligence
        // db.create_tool("IST", "choice", "cognitive", "Intelligence Structure Test").await?;
        // db.create_tool("CFIT", "choice", "cognitive", "Culture Fair Intelligence Test").await?;
        // db.create_tool("MATRICES", "choice", "cognitive", "APM/SPM/RPM (Raven's Matrices)").await?;
        // db.create_tool("WPT", "choice", "cognitive", "Wonderlic Personnel Test").await?;
        // db.create_tool("GATB", "choice", "cognitive", "General Aptitude Test Battery").await?;

        // [B] Personality & Preference
        // db.create_tool("EPPS", "pair", "personality", "Edwards Personal Preference Schedule").await?;
        // db.create_tool("PAPI", "pair", "personality", "PAPI Kostick").await?;
        // db.create_tool("DISC", "choice", "personality", "Dominance Influence Steadiness Compliance").await?;
        // db.create_tool("MBTI", "choice", "personality", "Myers-Briggs Type Indicator").await?;
        // db.create_tool("16PF", "choice", "personality", "Sixteen Personality Factor Questionnaire").await?;
        // db.create_tool("HEXACO", "choice", "personality", "HEXACO / Big Five").await?;
        db.create_tool("MSDT", "choice", "leadership", "Management Style Diagnosis Test").await?;
        db.create_tool("RMIB", "choice", "interest", "Rothwell Miller Interest Blank").await?;
        db.create_tool("RIASEC", "choice", "interest", "Holland Interest Test").await?;

        // [C] Performance & Clinical
        // db.create_tool("KRAEPELIN", "speed", "performance", "Kraepelin / Pauli Work Curve").await?;
        db.create_tool("KLOSE", "speed", "performance", "Tes Ketelitian / Klose").await?;
        db.create_tool("MMPI", "choice", "clinical", "Minnesota Multiphasic Personality Inventory").await?;
        db.create_tool("WARTEGG", "projective", "clinical", "Wartegg Test (8 Boxes)").await?;
        db.create_tool("PROJECTIVE", "projective", "clinical", "DAP / HTP / BAUM").await?;
    }

    println!("All tools seeded verification complete.");
    Ok(())
}

async fn seed_tiu(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("TIU", "choice", "cognitive", "Tes Intelegensia Umum").await?;
    seed_tiu_content(db, tool_id).await
}

async fn seed_tiu_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // 1. Verbal Ability (Synonyms/Antonims)
    let sub1 = db.create_subtest(tool_id, "Verbal Ability", 1, Some(300)).await?;
    db.create_question(sub1, "INSOMNIA = ...", "multiple_choice", serde_json::json!({"choices": ["Cemas", "Sedih", "Tidak bisa tidur", "Realita"], "correct": "Tidak bisa tidur"}), 1).await?;
    db.create_question(sub1, "BONAFIDE = ...", "multiple_choice", serde_json::json!({"choices": ["Tegar", "Jenis Bonsai", "Catatan", "Terpercaya"], "correct": "Terpercaya"}), 2).await?;
    db.create_question(sub1, "MOBILITAS >< ...", "multiple_choice", serde_json::json!({"choices": ["Stagnasi", "Gerak", "Transportasi", "Kendaraan"], "correct": "Stagnasi"}), 3).await?;

    // 2. Numerical Series
    let sub2 = db.create_subtest(tool_id, "Numerical Series", 2, Some(600)).await?;
    db.create_question(sub2, "2, 4, 8, 16, ...", "multiple_choice", serde_json::json!({"choices": ["32", "24", "18", "20"], "correct": "32"}), 1).await?;
    
    Ok(())
}

async fn seed_ist(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("IST", "choice", "cognitive", "Intelligence Structure Test").await?;
    seed_ist_content(db, tool_id).await
}

async fn seed_ist_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // 1. SE - Satzerganzung (Melengkapi Kalimat) - 6 min
    let sub_se = db.create_subtest(tool_id, "SE (Satzerganzung)", 1, Some(360)).await?;
    db.create_question(sub_se, "Seekor kuda mempunyai kesamaan terbanyak dengan seekor...", "multiple_choice", serde_json::json!({"choices": ["Kucing", "Bajing", "Kedelai", "Lembu", "Anjing"], "correct": "Lembu"}), 1).await?;
    db.create_question(sub_se, "Lawan hemat adalah...", "multiple_choice", serde_json::json!({"choices": ["Murah", "Kikir", "Boros", "Bernilai", "Kaya"], "correct": "Boros"}), 2).await?;

    // 2. WA - Wortausuahl (Melengkapi Kata) - 6 min
    let sub_wa = db.create_subtest(tool_id, "WA (Wortausuahl)", 2, Some(360)).await?;
    db.create_question(sub_wa, "Manakah kata yang tidak termasuk dalam kelompoknya?", "multiple_choice", serde_json::json!({"choices": ["Meja", "Kursi", "Burung", "Lemari", "Tempat Tidur"], "correct": "Burung"}), 1).await?;
    db.create_question(sub_wa, "Carilah kata yang berbeda!", "multiple_choice", serde_json::json!({"choices": ["Berjalan", "Mendengar", "Melihat", "Mencicipi", "Mencium"], "correct": "Berjalan"}), 2).await?;

    // 3. AN - Analogien (Persamaan Kata) - 7 min
    let sub_an = db.create_subtest(tool_id, "AN (Analogien)", 3, Some(420)).await?;
    db.create_question(sub_an, "Hutan : Pohon = Tembok : ?", "multiple_choice", serde_json::json!({"choices": ["Batu Bata", "Rumah", "Semen", "Putih", "Dinding"], "correct": "Batu Bata"}), 1).await?;
    db.create_question(sub_an, "Gelap : Terang = Basah : ?", "multiple_choice", serde_json::json!({"choices": ["Hujan", "Hari", "Lembab", "Angin", "Kering"], "correct": "Kering"}), 2).await?;

    // 4. GE - Gemeinsamkeiten (Sifat Dimiliki Bersama) - 8 min
    let sub_ge = db.create_subtest(tool_id, "GE (Gemeinsamkeiten)", 4, Some(480)).await?;
    db.create_question(sub_ge, "Apakah kesamaan antara Mawar dan Melati?", "multiple_choice", serde_json::json!({"choices": ["Bunga", "Merah", "Wangi", "Tumbuh", "Indah"], "correct": "Bunga"}), 1).await?;
    db.create_question(sub_ge, "Apakah kesamaan antara Mata dan Telinga?", "multiple_choice", serde_json::json!({"choices": ["Kepala", "Organ", "Panca Indera", "Wajah", "Dua"], "correct": "Panca Indera"}), 2).await?;

    // 5. RA - Rechenaufgaben (Berhitung) - 10 min
    let sub_ra = db.create_subtest(tool_id, "RA (Rechenaufgaben)", 5, Some(600)).await?;
    db.create_question(sub_ra, "Sebatang pensil harganya 25 rupiah. Berapakah harga 3 batang?", "multiple_choice", serde_json::json!({"choices": ["50", "60", "70", "75", "80"], "correct": "75"}), 1).await?;
    db.create_question(sub_ra, "Jika saya berjalan ke utara 6 km, lalu ke barat 8 km, berapa jarak saya dari titik awal?", "multiple_choice", serde_json::json!({"choices": ["10 km", "12 km", "14 km", "8 km", "6 km"], "correct": "10 km"}), 2).await?;

    // 6. ZR - Zahlenreihen (Deret Angka) - 10 min
    let sub_zr = db.create_subtest(tool_id, "ZR (Zahlenreihen)", 6, Some(600)).await?;
    db.create_question(sub_zr, "2, 4, 8, 16, 32, ...", "multiple_choice", serde_json::json!({"choices": ["48", "64", "50", "60", "62"], "correct": "64"}), 1).await?;
    db.create_question(sub_zr, "9, 7, 10, 8, 11, 9, 12, ...", "multiple_choice", serde_json::json!({"choices": ["10", "11", "12", "13", "14"], "correct": "10"}), 2).await?;

    // 7. FA - Figurenausuahl (Memilih Bentuk) - 7 min
    let sub_fa = db.create_subtest(tool_id, "FA (Figurenausuahl)", 7, Some(420)).await?;
    db.create_question(sub_fa, "Pilih potongan yang membentuk gambar utuh (Simulasi Gambar 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "A"}), 1).await?;
    db.create_question(sub_fa, "Pilih potongan yang membentuk gambar utuh (Simulasi Gambar 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "C"}), 2).await?;

    // 8. WU - Wurfelaufgaben (Latihan Balok) - 9 min
    let sub_wu = db.create_subtest(tool_id, "WU (Wurfelaufgaben)", 8, Some(540)).await?;
    db.create_question(sub_wu, "Manakah kubus yang benar jika diputar? (Simulasi Kubus 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "B"}), 1).await?;
    db.create_question(sub_wu, "Manakah kubus yang benar jika diputar? (Simulasi Kubus 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "D"}), 2).await?;

    // 9. ME - Merkaufgaben (Latihan Simbol) - 3 min hafalan, 6 min kerja
    let sub_me = db.create_subtest(tool_id, "ME (Merkaufgaben)", 9, Some(360)).await?;
    // In ME, user memorizes words associated with categories then answers queries
    db.create_question(sub_me, "Kata yang berawalan huruf 'J' termasuk kategori apa?", "multiple_choice", serde_json::json!({"choices": ["Bunga", "Alat", "Burung", "Kesenian", "Binatang"], "correct": "Kesenian"}), 1).await?;
    db.create_question(sub_me, "Kata yang berawalan huruf 'M' termasuk kategori apa?", "multiple_choice", serde_json::json!({"choices": ["Bunga", "Alat", "Burung", "Kesenian", "Binatang"], "correct": "Bunga"}), 2).await?;

    Ok(())
}

async fn seed_cfit(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("CFIT", "choice", "cognitive", "Culture Fair Intelligence Test").await?;
    seed_cfit_content(db, tool_id).await
}

async fn seed_cfit_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // 1. Series (Melanjutkan Gambar) - 3 min
    let sub1 = db.create_subtest(tool_id, "Subtest 1 (Series)", 1, Some(180)).await?;
    db.create_question(sub1, "Pilih gambar selanjutnya dalam seri ini (Simulasi Series 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "A"}), 1).await?;
    db.create_question(sub1, "Pilih gambar selanjutnya dalam seri ini (Simulasi Series 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "C"}), 2).await?;

    // 2. Classification (Berbeda dari Lainnya) - 4 min
    let sub2 = db.create_subtest(tool_id, "Subtest 2 (Classification)", 2, Some(240)).await?;
    db.create_question(sub2, "Manakah gambar yang berbeda? (Simulasi Class 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "B"}), 1).await?;
    db.create_question(sub2, "Manakah gambar yang berbeda? (Simulasi Class 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "E"}), 2).await?;

    // 3. Matrices (Melengkapi Matriks) - 3 min
    let sub3 = db.create_subtest(tool_id, "Subtest 3 (Matrices)", 3, Some(180)).await?;
    db.create_question(sub3, "Lengkapi bagian kosong matriks (Simulasi Matrix 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "C"}), 1).await?;
    db.create_question(sub3, "Lengkapi bagian kosong matriks (Simulasi Matrix 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "A"}), 2).await?;

    // 4. Conditions (Persyaratan Titik) - 2.5 min = 150s
    let sub4 = db.create_subtest(tool_id, "Subtest 4 (Conditions)", 4, Some(150)).await?;
    db.create_question(sub4, "Pilih gambar yang memenuhi kondisi titik (Simulasi Dot 1)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "D"}), 1).await?;
    db.create_question(sub4, "Pilih gambar yang memenuhi kondisi titik (Simulasi Dot 2)", "multiple_choice", serde_json::json!({"choices": ["A", "B", "C", "D", "E"], "correct": "B"}), 2).await?;

    Ok(())
}

async fn seed_kraepelin(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("KRAEPELIN", "speed", "performance", "Kraepelin / Pauli Work Curve").await?;
    seed_kraepelin_content(db, tool_id).await
}

async fn seed_kraepelin_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // 1. Latihan (Practice) - 1 min
    let sub1 = db.create_subtest(tool_id, "Latihan (Practice)", 1, Some(60)).await?;
    
    // Practice columns with realistic random patterns
    let practice_cols = vec![
        vec![4, 7, 2, 9, 5, 1, 8, 3, 6, 2, 7, 9, 4, 1, 5, 8, 3, 6, 2, 9],
        vec![2, 8, 3, 1, 9, 5, 7, 4, 6, 2, 8, 3, 5, 1, 9, 7, 4, 6, 3, 8],
        vec![5, 3, 8, 1, 6, 9, 2, 7, 4, 5, 3, 8, 1, 6, 9, 2, 7, 4, 5, 3],
    ];
    
    for (idx, nums) in practice_cols.iter().enumerate() {
        let col_json = serde_json::json!({"numbers": nums});
        db.create_question(sub1, &format!("Practice Column {}", idx + 1), "kraepelin_column", col_json, (idx + 1) as i64).await?;
    }

    // 2. Tes Utama (Main Test) - 20 min total
    // Standard Kraepelin has 45-60 columns, each with 40-50 numbers
    let sub2 = db.create_subtest(tool_id, "Tes Utama (Main)", 2, Some(1200)).await?;
    
    // Generate 45 realistic columns
    let main_test_columns = vec![
        vec![3, 7, 2, 8, 5, 1, 9, 4, 6, 2, 8, 3, 7, 1, 5, 9, 4, 6, 2, 8, 3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7],
        vec![5, 2, 9, 4, 7, 1, 6, 3, 8, 5, 2, 9, 4, 7, 1, 6, 3, 8, 5, 2, 9, 4, 7, 1, 6, 3, 8, 5, 2, 9, 4, 7, 1, 6, 3, 8, 5, 2, 9, 4],
        vec![8, 1, 5, 9, 3, 6, 2, 7, 4, 8, 1, 5, 9, 3, 6, 2, 7, 4, 8, 1, 5, 9, 3, 6, 2, 7, 4, 8, 1, 5, 9, 3, 6, 2, 7, 4, 8, 1, 5, 9],
        vec![2, 6, 4, 8, 1, 9, 5, 3, 7, 2, 6, 4, 8, 1, 9, 5, 3, 7, 2, 6, 4, 8, 1, 9, 5, 3, 7, 2, 6, 4, 8, 1, 9, 5, 3, 7, 2, 6, 4, 8],
        vec![7, 3, 9, 2, 5, 8, 1, 6, 4, 7, 3, 9, 2, 5, 8, 1, 6, 4, 7, 3, 9, 2, 5, 8, 1, 6, 4, 7, 3, 9, 2, 5, 8, 1, 6, 4, 7, 3, 9, 2],
        vec![4, 8, 1, 6, 9, 3, 7, 2, 5, 4, 8, 1, 6, 9, 3, 7, 2, 5, 4, 8, 1, 6, 9, 3, 7, 2, 5, 4, 8, 1, 6, 9, 3, 7, 2, 5, 4, 8, 1, 6],
        vec![9, 2, 7, 4, 1, 5, 8, 6, 3, 9, 2, 7, 4, 1, 5, 8, 6, 3, 9, 2, 7, 4, 1, 5, 8, 6, 3, 9, 2, 7, 4, 1, 5, 8, 6, 3, 9, 2, 7, 4],
        vec![1, 5, 3, 9, 6, 2, 8, 4, 7, 1, 5, 3, 9, 6, 2, 8, 4, 7, 1, 5, 3, 9, 6, 2, 8, 4, 7, 1, 5, 3, 9, 6, 2, 8, 4, 7, 1, 5, 3, 9],
        vec![6, 9, 2, 5, 8, 1, 4, 7, 3, 6, 9, 2, 5, 8, 1, 4, 7, 3, 6, 9, 2, 5, 8, 1, 4, 7, 3, 6, 9, 2, 5, 8, 1, 4, 7, 3, 6, 9, 2, 5],
        vec![3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7, 5, 1, 9, 4, 6, 2, 8, 3, 7, 5, 1],
        vec![8, 4, 1, 7, 3, 9, 5, 2, 6, 8, 4, 1, 7, 3, 9, 5, 2, 6, 8, 4, 1, 7, 3, 9, 5, 2, 6, 8, 4, 1, 7, 3, 9, 5, 2, 6, 8, 4, 1, 7],
        vec![2, 9, 6, 3, 7, 5, 1, 8, 4, 2, 9, 6, 3, 7, 5, 1, 8, 4, 2, 9, 6, 3, 7, 5, 1, 8, 4, 2, 9, 6, 3, 7, 5, 1, 8, 4, 2, 9, 6, 3],
        vec![5, 1, 8, 4, 2, 6, 9, 3, 7, 5, 1, 8, 4, 2, 6, 9, 3, 7, 5, 1, 8, 4, 2, 6, 9, 3, 7, 5, 1, 8, 4, 2, 6, 9, 3, 7, 5, 1, 8, 4],
        vec![7, 3, 5, 9, 1, 8, 2, 6, 4, 7, 3, 5, 9, 1, 8, 2, 6, 4, 7, 3, 5, 9, 1, 8, 2, 6, 4, 7, 3, 5, 9, 1, 8, 2, 6, 4, 7, 3, 5, 9],
        vec![4, 6, 2, 8, 5, 1, 7, 9, 3, 4, 6, 2, 8, 5, 1, 7, 9, 3, 4, 6, 2, 8, 5, 1, 7, 9, 3, 4, 6, 2, 8, 5, 1, 7, 9, 3, 4, 6, 2, 8],
        vec![9, 5, 1, 6, 3, 7, 4, 2, 8, 9, 5, 1, 6, 3, 7, 4, 2, 8, 9, 5, 1, 6, 3, 7, 4, 2, 8, 9, 5, 1, 6, 3, 7, 4, 2, 8, 9, 5, 1, 6],
        vec![1, 8, 4, 2, 9, 5, 3, 7, 6, 1, 8, 4, 2, 9, 5, 3, 7, 6, 1, 8, 4, 2, 9, 5, 3, 7, 6, 1, 8, 4, 2, 9, 5, 3, 7, 6, 1, 8, 4, 2],
        vec![6, 2, 9, 5, 1, 4, 8, 3, 7, 6, 2, 9, 5, 1, 4, 8, 3, 7, 6, 2, 9, 5, 1, 4, 8, 3, 7, 6, 2, 9, 5, 1, 4, 8, 3, 7, 6, 2, 9, 5],
        vec![3, 7, 1, 8, 4, 6, 2, 9, 5, 3, 7, 1, 8, 4, 6, 2, 9, 5, 3, 7, 1, 8, 4, 6, 2, 9, 5, 3, 7, 1, 8, 4, 6, 2, 9, 5, 3, 7, 1, 8],
        vec![8, 5, 3, 1, 7, 9, 6, 2, 4, 8, 5, 3, 1, 7, 9, 6, 2, 4, 8, 5, 3, 1, 7, 9, 6, 2, 4, 8, 5, 3, 1, 7, 9, 6, 2, 4, 8, 5, 3, 1],
        vec![2, 4, 7, 9, 3, 5, 1, 8, 6, 2, 4, 7, 9, 3, 5, 1, 8, 6, 2, 4, 7, 9, 3, 5, 1, 8, 6, 2, 4, 7, 9, 3, 5, 1, 8, 6, 2, 4, 7, 9],
        vec![5, 9, 2, 6, 8, 1, 4, 7, 3, 5, 9, 2, 6, 8, 1, 4, 7, 3, 5, 9, 2, 6, 8, 1, 4, 7, 3, 5, 9, 2, 6, 8, 1, 4, 7, 3, 5, 9, 2, 6],
        vec![7, 1, 4, 8, 2, 9, 5, 3, 6, 7, 1, 4, 8, 2, 9, 5, 3, 6, 7, 1, 4, 8, 2, 9, 5, 3, 6, 7, 1, 4, 8, 2, 9, 5, 3, 6, 7, 1, 4, 8],
        vec![4, 6, 9, 3, 5, 7, 2, 1, 8, 4, 6, 9, 3, 5, 7, 2, 1, 8, 4, 6, 9, 3, 5, 7, 2, 1, 8, 4, 6, 9, 3, 5, 7, 2, 1, 8, 4, 6, 9, 3],
        vec![9, 3, 6, 1, 8, 4, 7, 5, 2, 9, 3, 6, 1, 8, 4, 7, 5, 2, 9, 3, 6, 1, 8, 4, 7, 5, 2, 9, 3, 6, 1, 8, 4, 7, 5, 2, 9, 3, 6, 1],
        vec![1, 7, 5, 2, 9, 6, 3, 8, 4, 1, 7, 5, 2, 9, 6, 3, 8, 4, 1, 7, 5, 2, 9, 6, 3, 8, 4, 1, 7, 5, 2, 9, 6, 3, 8, 4, 1, 7, 5, 2],
        vec![6, 8, 2, 5, 1, 9, 4, 7, 3, 6, 8, 2, 5, 1, 9, 4, 7, 3, 6, 8, 2, 5, 1, 9, 4, 7, 3, 6, 8, 2, 5, 1, 9, 4, 7, 3, 6, 8, 2, 5],
        vec![3, 5, 8, 4, 7, 2, 9, 1, 6, 3, 5, 8, 4, 7, 2, 9, 1, 6, 3, 5, 8, 4, 7, 2, 9, 1, 6, 3, 5, 8, 4, 7, 2, 9, 1, 6, 3, 5, 8, 4],
        vec![8, 2, 1, 7, 4, 6, 9, 3, 5, 8, 2, 1, 7, 4, 6, 9, 3, 5, 8, 2, 1, 7, 4, 6, 9, 3, 5, 8, 2, 1, 7, 4, 6, 9, 3, 5, 8, 2, 1, 7],
        vec![2, 9, 4, 6, 3, 8, 1, 5, 7, 2, 9, 4, 6, 3, 8, 1, 5, 7, 2, 9, 4, 6, 3, 8, 1, 5, 7, 2, 9, 4, 6, 3, 8, 1, 5, 7, 2, 9, 4, 6],
        vec![5, 7, 3, 9, 1, 4, 8, 2, 6, 5, 7, 3, 9, 1, 4, 8, 2, 6, 5, 7, 3, 9, 1, 4, 8, 2, 6, 5, 7, 3, 9, 1, 4, 8, 2, 6, 5, 7, 3, 9],
        vec![7, 4, 2, 8, 5, 1, 6, 9, 3, 7, 4, 2, 8, 5, 1, 6, 9, 3, 7, 4, 2, 8, 5, 1, 6, 9, 3, 7, 4, 2, 8, 5, 1, 6, 9, 3, 7, 4, 2, 8],
        vec![4, 1, 6, 3, 9, 7, 2, 5, 8, 4, 1, 6, 3, 9, 7, 2, 5, 8, 4, 1, 6, 3, 9, 7, 2, 5, 8, 4, 1, 6, 3, 9, 7, 2, 5, 8, 4, 1, 6, 3],
        vec![9, 6, 1, 5, 2, 8, 4, 7, 3, 9, 6, 1, 5, 2, 8, 4, 7, 3, 9, 6, 1, 5, 2, 8, 4, 7, 3, 9, 6, 1, 5, 2, 8, 4, 7, 3, 9, 6, 1, 5],
        vec![1, 3, 8, 2, 7, 5, 9, 4, 6, 1, 3, 8, 2, 7, 5, 9, 4, 6, 1, 3, 8, 2, 7, 5, 9, 4, 6, 1, 3, 8, 2, 7, 5, 9, 4, 6, 1, 3, 8, 2],
        vec![6, 5, 9, 1, 4, 3, 7, 8, 2, 6, 5, 9, 1, 4, 3, 7, 8, 2, 6, 5, 9, 1, 4, 3, 7, 8, 2, 6, 5, 9, 1, 4, 3, 7, 8, 2, 6, 5, 9, 1],
        vec![3, 8, 4, 7, 1, 9, 2, 6, 5, 3, 8, 4, 7, 1, 9, 2, 6, 5, 3, 8, 4, 7, 1, 9, 2, 6, 5, 3, 8, 4, 7, 1, 9, 2, 6, 5, 3, 8, 4, 7],
        vec![8, 1, 5, 9, 6, 2, 4, 3, 7, 8, 1, 5, 9, 6, 2, 4, 3, 7, 8, 1, 5, 9, 6, 2, 4, 3, 7, 8, 1, 5, 9, 6, 2, 4, 3, 7, 8, 1, 5, 9],
        vec![2, 7, 3, 5, 8, 4, 1, 9, 6, 2, 7, 3, 5, 8, 4, 1, 9, 6, 2, 7, 3, 5, 8, 4, 1, 9, 6, 2, 7, 3, 5, 8, 4, 1, 9, 6, 2, 7, 3, 5],
        vec![5, 9, 1, 4, 7, 6, 3, 2, 8, 5, 9, 1, 4, 7, 6, 3, 2, 8, 5, 9, 1, 4, 7, 6, 3, 2, 8, 5, 9, 1, 4, 7, 6, 3, 2, 8, 5, 9, 1, 4],
        vec![7, 2, 6, 8, 3, 1, 5, 4, 9, 7, 2, 6, 8, 3, 1, 5, 4, 9, 7, 2, 6, 8, 3, 1, 5, 4, 9, 7, 2, 6, 8, 3, 1, 5, 4, 9, 7, 2, 6, 8],
        vec![4, 8, 2, 1, 9, 5, 7, 6, 3, 4, 8, 2, 1, 9, 5, 7, 6, 3, 4, 8, 2, 1, 9, 5, 7, 6, 3, 4, 8, 2, 1, 9, 5, 7, 6, 3, 4, 8, 2, 1],
        vec![9, 3, 7, 2, 5, 8, 6, 1, 4, 9, 3, 7, 2, 5, 8, 6, 1, 4, 9, 3, 7, 2, 5, 8, 6, 1, 4, 9, 3, 7, 2, 5, 8, 6, 1, 4, 9, 3, 7, 2],
        vec![1, 6, 4, 9, 2, 7, 3, 8, 5, 1, 6, 4, 9, 2, 7, 3, 8, 5, 1, 6, 4, 9, 2, 7, 3, 8, 5, 1, 6, 4, 9, 2, 7, 3, 8, 5, 1, 6, 4, 9],
        vec![6, 4, 8, 3, 1, 9, 5, 2, 7, 6, 4, 8, 3, 1, 9, 5, 2, 7, 6, 4, 8, 3, 1, 9, 5, 2, 7, 6, 4, 8, 3, 1, 9, 5, 2, 7, 6, 4, 8, 3],
    ];
    
    for (idx, nums) in main_test_columns.iter().enumerate() {
        let col_data = serde_json::json!({"numbers": nums});
        db.create_question(sub2, &format!("Column {}", idx + 1), "kraepelin_column", col_data, (idx + 1) as i64).await?;
    }

    Ok(())
}

async fn seed_matrices(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("MATRICES", "choice", "cognitive", "APM/SPM/RPM (Raven's Matrices)").await?;
    seed_matrices_content(db, tool_id).await
}

async fn seed_matrices_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // Raven's Progressive Matrices typically has 5 sets with 12 questions each
    // Set A: Easier patterns (continuous patterns)
    let set_a = db.create_subtest(tool_id, "Set A", 1, Some(300)).await?;
    db.create_question(set_a, "Complete the pattern (A1)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6"], "correct": "3"}), 1).await?;
    db.create_question(set_a, "Complete the pattern (A2)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6"], "correct": "2"}), 2).await?;

    // Set B: Medium difficulty (analogies)
    let set_b = db.create_subtest(tool_id, "Set B", 2, Some(360)).await?;
    db.create_question(set_b, "Complete the pattern (B1)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6"], "correct": "4"}), 1).await?;
    db.create_question(set_b, "Complete the pattern (B2)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6"], "correct": "5"}), 2).await?;

    // Set C: Progressive difficulty (patterns)
    let set_c = db.create_subtest(tool_id, "Set C", 3, Some(420)).await?;
    db.create_question(set_c, "Complete the pattern (C1)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "6"}), 1).await?;
    db.create_question(set_c, "Complete the pattern (C2)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "3"}), 2).await?;

    // Set D: Advanced patterns
    let set_d = db.create_subtest(tool_id, "Set D", 4, Some(480)).await?;
    db.create_question(set_d, "Complete the pattern (D1)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "7"}), 1).await?;
    db.create_question(set_d, "Complete the pattern (D2)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "1"}), 2).await?;

    // Set E: Most difficult (complex patterns and systems)
    let set_e = db.create_subtest(tool_id, "Set E", 5, Some(540)).await?;
    db.create_question(set_e, "Complete the pattern (E1)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "2"}), 1).await?;
    db.create_question(set_e, "Complete the pattern (E2)", "multiple_choice", serde_json::json!({"choices": ["1", "2", "3", "4", "5", "6", "7", "8"], "correct": "8"}), 2).await?;

    Ok(())
}

async fn seed_wpt(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("WPT", "choice", "cognitive", "Wonderlic Personnel Test").await?;
    seed_wpt_content(db, tool_id).await
}

async fn seed_wpt_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // WPT has 50 questions total, 12 minute time limit (720 seconds)
    let subtest = db.create_subtest(tool_id, "Full Test", 1, Some(720)).await?;
    
    // Cognitive mix: Verbal (15), Numerical (20), Abstract (15)
    db.create_question(subtest, "The opposite of INCREASE is:", "multiple_choice", serde_json::json!({"choices": ["reduce", "expand", "grow", "add"],  "correct": "reduce"}), 1).await?;
    db.create_question(subtest, "15 + 27 = ?", "multiple_choice", serde_json::json!({"choices": ["40", "42", "41", "43"], "correct": "42"}), 2).await?;
    db.create_question(subtest, "ANXIOUS is to WORRIED as CALM is to:", "multiple_choice", serde_json::json!({"choices": ["nervous", "peaceful", "angry", "sad"], "correct": "peaceful"}), 3).await?;
    db.create_question(subtest, "84 ÷ 7 = ?", "multiple_choice", serde_json::json!({"choices": ["11", "12", "13", "14"], "correct": "12"}), 4).await?;
    db.create_question(subtest, "Which does not belong? DOG, CAT, BIRD, CHAIR", "multiple_choice", serde_json::json!({"choices": ["DOG", "CAT", "BIRD", "CHAIR"], "correct": "CHAIR"}), 5).await?;
    db.create_question(subtest, "What is 30% of 200?", "multiple_choice", serde_json::json!({"choices": ["50", "60", "70", "80"], "correct": "60"}), 6).await?;
    db.create_question(subtest, "If pencils cost $0.25 each, how many can you buy with $3.00?", "multiple_choice", serde_json::json!({"choices": ["10", "11", "12", "13"], "correct": "12"}), 7).await?;
    db.create_question(subtest, "What number comes next: 2, 6, 18, 54, ___?", "multiple_choice", serde_json::json!({"choices": ["108", "162", "216", "270"], "correct": "162"}), 8).await?;
    db.create_question(subtest, "46 × 3 = ?", "multiple_choice", serde_json::json!({"choices": ["136", "137", "138", "139"], "correct": "138"}), 9).await?;
    db.create_question(subtest, "The days of the week: Sun, Mon, Tue, Wed, Thu, ___", "multiple_choice", serde_json::json!({"choices": ["Sat", "Fri", "Weekend", "Month"], "correct": "Fri"}), 10).await?;
    db.create_question(subtest, "What is 1/4 of 80?", "multiple_choice", serde_json::json!({"choices": ["15", "18", "20", "25"], "correct": "20"}), 11).await?;
    db.create_question(subtest, "If all Bloops are Razzies and all Razzies are Lazzies, then all Bloops are Lazzies?", "multiple_choice", serde_json::json!({"choices": ["True", "False", "Cannot determine"], "correct": "True"}), 12).await?;
    db.create_question(subtest, "156 + 89 = ?", "multiple_choice", serde_json::json!({"choices": ["243", "244", "245", "246"], "correct": "245"}), 13).await?;
    db.create_question(subtest, "Complete: AZ, BY, CX, DW, ___", "multiple_choice", serde_json::json!({"choices": ["EV", "EU", "FV", "EW"], "correct": "EV"}), 14).await?;
    db.create_question(subtest, "25 × 8 = ?", "multiple_choice", serde_json::json!({"choices": ["180", "190", "200", "210"], "correct": "200"}), 15).await?;
    db.create_question(subtest, "What is the next number: 100, 98, 94, 88, 80, ___?", "multiple_choice", serde_json::json!({"choices": ["68", "70", "72", "74"], "correct": "70"}), 16).await?;
    db.create_question(subtest, "If BOOK is CPPL, how is DOOR written?", "multiple_choice", serde_json::json!({"choices": ["EPPS", "DOPS", "EPPR", "DPPR"], "correct": "EPPS"}), 17).await?;
    db.create_question(subtest, "234 - 178 = ?", "multiple_choice", serde_json::json!({"choices": ["54", "55", "56", "57"], "correct": "56"}), 18).await?;
    db.create_question(subtest, "What comes next: 1, 1, 2, 3, 5, 8, ___?", "multiple_choice", serde_json::json!({"choices": ["11", "12", "13", "14"], "correct": "13"}), 19).await?;
    db.create_question(subtest, "96 ÷ 12 = ?", "multiple_choice", serde_json::json!({"choices": ["6", "7", "8", "9"], "correct": "8"}), 20).await?;
    db.create_question(subtest, "A worker earns $18/hr. How much in 6.5 hours?", "multiple_choice", serde_json::json!({"choices": ["$108", "$114", "$117", "$120"], "correct": "$117"}), 21).await?;
    db.create_question(subtest, "What is 15% of 60?", "multiple_choice", serde_json::json!({"choices": ["7", "8", "9", "10"], "correct": "9"}), 22).await?;
    db.create_question(subtest, "What is half of 136?", "multiple_choice", serde_json::json!({"choices": ["64", "66", "68", "70"], "correct": "68"}), 23).await?;
    db.create_question(subtest, "Which number doesn't belong: 2, 3, 5, 7, 9, 11?", "multiple_choice", serde_json::json!({"choices": ["2", "3", "5", "9"], "correct": "9"}), 24).await?;
    db.create_question(subtest, "45 + 67 - 23 = ?", "multiple_choice", serde_json::json!({"choices": ["87", "88", "89", "90"], "correct": "89"}), 25).await?;
    db.create_question(subtest, "What comes next: 1, 4, 9, 16, 25, ___?", "multiple_choice", serde_json::json!({"choices": ["30", "32", "34", "36"], "correct": "36"}), 26).await?;
    db.create_question(subtest, "If car travels 60 mph, how far in 2.5 hours?", "multiple_choice", serde_json::json!({"choices": ["120 mi", "130 mi", "140 mi", "150 mi"], "correct": "150 mi"}), 27).await?;
    db.create_question(subtest, "The third month of the year is:", "multiple_choice", serde_json::json!({"choices": ["January", "February", "March", "April"], "correct": "March"}), 28).await?;
    db.create_question(subtest, "Continue: 2, 5, 11, 23, 47, ___?", "multiple_choice", serde_json::json!({"choices": ["91", "93", "95", "97"], "correct": "95"}), 29).await?;
    db.create_question(subtest, "How many months have 30 days?", "multiple_choice", serde_json::json!({"choices": ["2", "3", "4", "5"], "correct": "4"}), 30).await?;
    db.create_question(subtest, "Opposite of GENEROUS is:", "multiple_choice", serde_json::json!({"choices": ["kind", "selfish", "friendly", "happy"], "correct": "selfish"}), 31).await?;
    db.create_question(subtest, "48 cookies in boxes of 6. How many boxes?", "multiple_choice", serde_json::json!({"choices": ["6", "7", "8", "9"], "correct": "8"}), 32).await?;
    db.create_question(subtest, "If tomorrow is Wed, what day was yesterday?", "multiple_choice", serde_json::json!({"choices": ["Sun", "Mon", "Tue", "Thu"], "correct": "Mon"}), 33).await?;
    db.create_question(subtest, "What is 1/2 of 1/4 of 1/10 of 800?", "multiple_choice", serde_json::json!({"choices": ["8", "10", "12", "15"], "correct": "10"}), 34).await?;
    db.create_question(subtest, "Which is different: North, South, East, West, Equator", "multiple_choice", serde_json::json!({"choices": ["North", "South", "East", "Equator"], "correct": "Equator"}), 35).await?;
    db.create_question(subtest, "Pattern: O, T, T, F, F, S, S, E, N, ___?", "multiple_choice", serde_json::json!({"choices": ["E", "T", "N", "F"], "correct": "T"}), 36).await?;
    db.create_question(subtest, "Continue: 3, 7, 15, 31, ___?", "multiple_choice", serde_json::json!({"choices": ["55", "59", "62", "63"], "correct": "63"}), 37).await?;
    db.create_question(subtest, "A cube has how many faces?", "multiple_choice", serde_json::json!({"choices": ["4", "5", "6", "8"], "correct": "6"}), 38).await?;
    db.create_question(subtest, "If 5 machines make 5 widgets in 5 min, 100 machines make 100 in?", "multiple_choice", serde_json::json!({"choices": ["1 min", "5 min", "10 min", "100 min"], "correct": "5 min"}), 39).await?;
    db.create_question(subtest, "Same meaning as BEGIN:", "multiple_choice", serde_json::json!({"choices": ["start", "end", "middle", "finish"], "correct": "start"}), 40).await?;
    db.create_question(subtest, "Next letter: A, D, G, J, M, ___?", "multiple_choice", serde_json::json!({"choices": ["N", "O", "P", "Q"], "correct": "P"}), 41).await?;
    db.create_question(subtest, "If you face north, turn left 90°, then right 180°, you face:", "multiple_choice", serde_json::json!({"choices": ["North", "South", "East", "West"], "correct": "East"}), 42).await?;
    db.create_question(subtest, "Clock strikes 6 in 5 sec. How long for 12?", "multiple_choice", serde_json::json!({"choices": ["10 sec", "11 sec", "12 sec", "13 sec"], "correct": "11 sec"}), 43).await?;
    db.create_question(subtest, "123 - 67 = ?", "multiple_choice", serde_json::json!({"choices": ["54", "55", "56", "57"], "correct": "56"}), 44).await?;
    db.create_question(subtest, "72 ÷ 9 = ?", "multiple_choice", serde_json::json!({"choices": ["6", "7", "8", "9"], "correct": "8"}), 45).await?;
    db.create_question(subtest, "What number missing: 5, 10, 15, ___, 25?", "multiple_choice", serde_json::json!({"choices": ["17", "18", "19", "20"], "correct": "20"}), 46).await?;
    db.create_question(subtest, "If 3 apples cost $1.50, 7 apples cost?", "multiple_choice", serde_json::json!({"choices": ["$3.00", "$3.50", "$4.00", "$4.50"], "correct": "$3.50"}), 47).await?;
    db.create_question(subtest, "Train leaves 9:30 AM, arrives 11:45 AM. Trip time?", "multiple_choice", serde_json::json!({"choices": ["2:15", "2:00", "2:30", "3:00"], "correct": "2:15"}), 48).await?;
    db.create_question(subtest, "Rearrange CIFAIPC to get:", "multiple_choice", serde_json::json!({"choices": ["City", "Animal", "Ocean", "Country"], "correct": "Ocean"}), 49).await?;
    db.create_question(subtest, "Clock at 3:15. Angle between hands?", "multiple_choice", serde_json::json!({"choices": ["0°", "7.5°", "15°", "30°"], "correct": "7.5°"}), 50).await?;

    Ok(())
}


async fn seed_disc(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("DISC", "choice", "personality", "Dominance Influence Steadiness Compliance").await?;
    seed_disc_content(db, tool_id).await
}

async fn seed_disc_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    let subtest = db.create_subtest(tool_id, "DISC Assessment", 1, Some(600)).await?;
    
    db.create_question(subtest, "Which word best describes you?", "multiple_choice", serde_json::json!({"choices": ["Assertive (D)", "Enthusiastic (I)", "Patient (S)", "Careful (C)"], "correct": ""}), 1).await?;
    db.create_question(subtest, "In a group setting, you tend to be:", "multiple_choice", serde_json::json!({"choices": ["Directing (D)", "Persuading (I)", "Supporting (S)", "Analyzing (C)"], "correct": ""}), 2).await?;
    db.create_question(subtest, "Your approach to tasks is typically:", "multiple_choice", serde_json::json!({"choices": ["Results-focused (D)", "People-focused (I)", "Team-focused (S)", "Detail-focused (C)"], "correct": ""}), 3).await?;
    db.create_question(subtest, "When making decisions, you are:", "multiple_choice", serde_json::json!({"choices": ["Decisive (D)", "Optimistic (I)", "Considerate (S)", "Methodical (C)"], "correct": ""}), 4).await?;
    db.create_question(subtest, "Your communication style is:", "multiple_choice", serde_json::json!({"choices": ["Direct (D)", "Expressive (I)", "Calm (S)", "Precise (C)"], "correct": ""}), 5).await?;
    db.create_question(subtest, "You prefer environments that are:", "multiple_choice", serde_json::json!({"choices": ["Challenging (D)", "Dynamic (I)", "Stable (S)", "Structured (C)"], "correct": ""}), 6).await?;
    db.create_question(subtest, "Your greatest strength is:", "multiple_choice", serde_json::json!({"choices": ["Leadership (D)", "Inspiration (I)", "Loyalty (S)", "Accuracy (C)"], "correct": ""}), 7).await?;
    db.create_question(subtest, "When facing problems, you:", "multiple_choice", serde_json::json!({"choices": ["Take charge (D)", "Brainstorm ideas (I)", "Listen carefully (S)", "Gather data (C)"], "correct": ""}), 8).await?;
    db.create_question(subtest, "You are most comfortable when:", "multiple_choice", serde_json::json!({"choices": ["In control (D)", "Collaborating (I)", "Helping others (S)", "Following procedures (C)"], "correct": ""}), 9).await?;
    db.create_question(subtest, "Your work pace is:", "multiple_choice", serde_json::json!({"choices": ["Fast-paced (D)", "Varied (I)", "Steady (S)", "Deliberate (C)"], "correct": ""}), 10).await?;
    db.create_question(subtest, "You value:", "multiple_choice", serde_json::json!({"choices": ["Achievement (D)", "Recognition (I)", "Harmony (S)", "Quality (C)"], "correct": ""}), 11).await?;
    db.create_question(subtest, "Others see you as:", "multiple_choice", serde_json::json!({"choices": ["Confident (D)", "Outgoing (I)", "Dependable (S)", "Systematic (C)"], "correct": ""}), 12).await?;
    db.create_question(subtest, "You respond to stress by:", "multiple_choice", serde_json::json!({"choices": ["Confronting it (D)", "Talking it out (I)", "Avoiding conflict (S)", "Analyzing it (C)"], "correct": ""}), 13).await?;
    db.create_question(subtest, "Your motivation comes from:", "multiple_choice", serde_json::json!({"choices": ["Winning (D)", "Popularity (I)", "Acceptance (S)", "Correctness (C)"], "correct": ""}), 14).await?;
    db.create_question(subtest, "When working on projects, you:", "multiple_choice", serde_json::json!({"choices": ["Drive results (D)", "Generate enthusiasm (I)", "Provide support (S)", "Ensure accuracy (C)"], "correct": ""}), 15).await?;
    db.create_question(subtest, "You prefer to:", "multiple_choice", serde_json::json!({"choices": ["Lead (D)", "Influence (I)", "Follow (S)", "Organize (C)"], "correct": ""}), 16).await?;
    db.create_question(subtest, "Your thinking style is:", "multiple_choice", serde_json::json!({"choices": ["Strategic (D)", "Creative (I)", "Practical (S)", "Logical (C)"], "correct": ""}), 17).await?;
    db.create_question(subtest, "You fear:", "multiple_choice", serde_json::json!({"choices": ["Being taken advantage of (D)", "Rejection (I)", "Change (S)", "Criticism (C)"], "correct": ""}), 18).await?;
    db.create_question(subtest, "In conversations, you tend to:", "multiple_choice", serde_json::json!({"choices": ["Be blunt (D)", "Be animated (I)", "Be agreeable (S)", "Be factual (C)"], "correct": ""}), 19).await?;
    db.create_question(subtest, "You make an impression by being:", "multiple_choice", serde_json::json!({"choices": ["Powerful (D)", "Charming (I)", "Sincere (S)", "Competent (C)"], "correct": ""}), 20).await?;
    db.create_question(subtest, "Your focus is on:", "multiple_choice", serde_json::json!({"choices": ["Goals (D)", "People (I)", "Relationships (S)", "Standards (C)"], "correct": ""}), 21).await?;
    db.create_question(subtest, "You learn best through:", "multiple_choice", serde_json::json!({"choices": ["Doing (D)", "Discussing (I)", "Observing (S)", "Reading (C)"], "correct": ""}), 22).await?;
    db.create_question(subtest, "Your natural tendency is to be:", "multiple_choice", serde_json::json!({"choices": ["Competitive (D)", "Sociable (I)", "Cooperative (S)", "Cautious (C)"], "correct": ""}), 23).await?;
    db.create_question(subtest, "You are described as:", "multiple_choice", serde_json::json!({"choices": ["Strong-willed (D)", "Enthusiastic (I)", "Easygoing (S)", "Perfectionist (C)"], "correct": ""}), 24).await?;

    Ok(())
}

async fn seed_mbti(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("MBTI", "choice", "personality", "Myers-Briggs Type Indicator").await?;
    seed_mbti_content(db, tool_id).await
}

async fn seed_mbti_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    let subtest = db.create_subtest(tool_id, "MBTI Questionnaire", 1, Some(1200)).await?;
    
    // E/I Questions (Extraversion vs Introversion) - 23 questions
    db.create_question(subtest, "At a party, you:", "multiple_choice", serde_json::json!({"choices": ["Interact with many, even strangers", "Interact with a few close friends"], "correct": ""}), 1).await?;
    db.create_question(subtest, "You feel more energized:", "multiple_choice", serde_json::json!({"choices": ["After spending time with groups", "After quiet time alone"], "correct": ""}), 2).await?;
    db.create_question(subtest, "You prefer to:", "multiple_choice", serde_json::json!({"choices": ["Think out loud with others", "Think things through privately"], "correct": ""}), 3).await?;
    db.create_question(subtest, "In conversations, you:", "multiple_choice", serde_json::json!({"choices": ["Share readily and quickly", "Take time before sharing"], "correct": ""}), 4).await?;
    db.create_question(subtest, "You are seen as:", "multiple_choice", serde_json::json!({"choices": ["Outgoing and talkative", "Reserved and quiet"], "correct": ""}), 5).await?;
    db.create_question(subtest, "You prefer:", "multiple_choice", serde_json::json!({"choices": ["A wide circle of friends", "A small group of close friends"], "correct": ""}), 6).await?;
    
    // S/N Questions (Sensing vs Intuition) - 23 questions
    db.create_question(subtest, "You focus on:", "multiple_choice", serde_json::json!({"choices": ["Facts and concrete details", "Possibilities and meanings"], "correct": ""}), 7).await?;
    db.create_question(subtest, "You prefer:", "multiple_choice", serde_json::json!({"choices": ["Practical and realistic approaches", "Imaginative and innovative approaches"], "correct": ""}), 8).await?;
    db.create_question(subtest, "You trust:", "multiple_choice", serde_json::json!({"choices": ["Experience and what works", "Hunches and gut feelings"], "correct": ""}), 9).await?;
    db.create_question(subtest, "You value:", "multiple_choice", serde_json::json!({"choices": ["Proven methods", "New possibilities"], "correct": ""}), 10).await?;
    db.create_question(subtest, "You are:", "multiple_choice", serde_json::json!({"choices": ["Detail-oriented", "Big-picture oriented"], "correct": ""}), 11).await?;
    db.create_question(subtest, "You prefer instructions that are:", "multiple_choice", serde_json::json!({"choices": ["Specific and literal", "General and metaphorical"], "correct": ""}), 12).await?;
    
    // T/F Questions (Thinking vs Feeling) - 24 questions
    db.create_question(subtest, "When making decisions, you rely more on:", "multiple_choice", serde_json::json!({"choices": ["Logic and objectivity", "Values and empathy"], "correct": ""}), 13).await?;
    db.create_question(subtest, "You value:", "multiple_choice", serde_json::json!({"choices": ["Truth and fairness", "Harmony and compassion"], "correct": ""}), 14).await?;
    db.create_question(subtest, "In disagreements, you:", "multiple_choice", serde_json::json!({"choices": ["Stay firm on principles", "Seek to preserve relationships"], "correct": ""}), 15).await?;
    db.create_question(subtest, "You are more:", "multiple_choice", serde_json::json!({"choices": ["Analytical", "Sympathetic"], "correct": ""}), 16).await?;
    db.create_question(subtest, "You prefer to be:", "multiple_choice", serde_json::json!({"choices": ["Just and fair", "Caring and tactful"], "correct": ""}), 17).await?;
    db.create_question(subtest, "When helping someone, you:", "multiple_choice", serde_json::json!({"choices": ["Offer solutions", "Offer support"], "correct": ""}), 18).await?;
    
    // J/P Questions (Judging vs Perceiving) - 23 questions
    db.create_question(subtest, "You prefer:", "multiple_choice", serde_json::json!({"choices": ["Planned and organized", "Spontaneous and flexible"], "correct": ""}), 19).await?;
    db.create_question(subtest, "You like:", "multiple_choice", serde_json::json!({"choices": ["Settling things decided", "Keeping options open"], "correct": ""}), 20).await?;
    db.create_question(subtest, "Your workspace is:", "multiple_choice", serde_json::json!({"choices": ["Organized and neat", "Flexible and casual"], "correct": ""}), 21).await?;
    db.create_question(subtest, "You work best:", "multiple_choice", serde_json::json!({"choices": ["With deadlines and structure", "With freedom and flexibility"], "correct": ""}), 22).await?;
    db.create_question(subtest, "You prefer to:", "multiple_choice", serde_json::json!({"choices": ["Make lists and follow them", "Go with the flow"], "correct": ""}), 23).await?;
    db.create_question(subtest, "You feel better when things are:", "multiple_choice", serde_json::json!({"choices": ["Decided and settled", "Open to change"], "correct": ""}), 24).await?;
    
    // Continue with more balanced questions across all dimensions (shortened for practical reasons, but maintaining realistic structure)
    db.create_question(subtest, "Social gatherings:", "multiple_choice", serde_json::json!({"choices": ["Are fun and energizing", "Can be draining"], "correct": ""}), 25).await?;
    db.create_question(subtest, "You learn best by:", "multiple_choice", serde_json::json!({"choices": ["Hands-on practice", "Reading about concepts"], "correct": ""}), 26).await?;
    db.create_question(subtest, "In conflicts, you prioritize:", "multiple_choice", serde_json::json!({"choices": ["Being right", "Being understanding"], "correct": ""}), 27).await?;
    db.create_question(subtest, "Your ideal weekend:", "multiple_choice", serde_json::json!({"choices": ["Well-planned activities", "See where the day takes you"], "correct": ""}), 28).await?;
    db.create_question(subtest, "You initiate conversations:", "multiple_choice", serde_json::json!({"choices": ["Often and easily", "Rarely, when necessary"], "correct": ""}), 29).await?;
    db.create_question(subtest, "You notice:", "multiple_choice", serde_json::json!({"choices": ["What is actually there", "What could be there"], "correct": ""}), 30).await?;
    
    // Add 63 more questions following MBTI patterns (simplified for this implementation)
    for i in 31..=93 {
        let dimension = match i % 4 {
            1 => ("Do you recharge by:", "Being with people", "Being alone"),
            2 => ("You focus on:", "The present reality", "Future possibilities"),
            3 => ("You decide based on:", "Head and logic", "Heart and values"),
            0 => ("You prefer:", "Plans and schedules", "Flexibility and options"),
            _ => ("Question:", "Option A", "Option B")
        };
        db.create_question(subtest, dimension.0, "multiple_choice", 
            serde_json::json!({"choices": [dimension.1, dimension.2], "correct": ""}), i.into()).await?;
    }

    Ok(())
}

async fn seed_gatb(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("GATB", "choice", "cognitive", "General Aptitude Test Battery").await?;
    seed_gatb_content(db, tool_id).await
}

async fn seed_gatb_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // GATB has 8 subtests measuring different aptitudes
    
    // 1. Name Comparison - 6 min, perceptual speed (sample 10 of 100)
    let sub1 = db.create_subtest(tool_id, "Name Comparison", 1, Some(360)).await?;
    for i in 1..=10 {
        db.create_question(sub1, &format!("Are these names the same? JOHN SMITH vs JOHN SMYTH"), "multiple_choice", 
            serde_json::json!({"choices": ["Same", "Different"], "correct": "Different"}), i).await?;
    }
    
    // 2. Computation - 6 min, numerical ability (sample 10 of 50)
    let sub2 = db.create_subtest(tool_id, "Computation", 2, Some(360)).await?;
    db.create_question(sub2, "45 + 67 = ?", "multiple_choice", serde_json::json!({"choices": ["110", "111", "112", "113"], "correct": "112"}), 1).await?;
    db.create_question(sub2, "234 - 89 = ?", "multiple_choice", serde_json::json!({"choices": ["143", "144", "145", "146"], "correct": "145"}), 2).await?;
    db.create_question(sub2, "15 × 12 = ?", "multiple_choice", serde_json::json!({"choices": ["170", "175", "180", "185"], "correct": "180"}), 3).await?;
    db.create_question(sub2, "144 ÷ 12 = ?", "multiple_choice", serde_json::json!({"choices": ["10", "11", "12", "13"], "correct": "12"}), 4).await?;
    db.create_question(sub2, "What is 25% of 80?", "multiple_choice", serde_json::json!({"choices": ["15", "18", "20", "22"], "correct": "20"}), 5).await?;
    for i in 6..=10 {
        db.create_question(sub2, &format!("Math problem {}", i), "multiple_choice", 
            serde_json::json!({"choices": ["A", "B", "C", "D"], "correct": "A"}), i.into()).await?;
    }
    
    // 3. Three-Dimensional Space - 6 min, spatial ability (sample 10 of 40)
    let sub3 = db.create_subtest(tool_id, "Three-Dimensional Space", 3, Some(360)).await?;
    for i in 1..=10 {
        db.create_question(sub3, "Which 3D shape matches the unfolded pattern?", "multiple_choice", 
            serde_json::json!({"choices": ["Cube", "Pyramid", "Cylinder", "Cone"], "correct": "Cube"}), i).await?;
    }
    
    // 4. Vocabulary - 6 min, verbal ability (sample 10 of 60)
    let sub4 = db.create_subtest(tool_id, "Vocabulary", 4, Some(360)).await?;
    db.create_question(sub4, "RAPID means:", "multiple_choice", serde_json::json!({"choices": ["Quick", "Slow", "Careful", "Late"], "correct": "Quick"}), 1).await?;
    db.create_question(sub4, "ANCIENT means:", "multiple_choice", serde_json::json!({"choices": ["New", "Old", "Big", "Small"], "correct": "Old"}), 2).await?;
    db.create_question(sub4, "BRAVE means:", "multiple_choice", serde_json::json!({"choices": ["Fearful", "Courageous", "Weak", "Tired"], "correct": "Courageous"}), 3).await?;
    db.create_question(sub4, "ENORMOUS means:", "multiple_choice", serde_json::json!({"choices": ["Tiny", "Huge", "Average", "Narrow"], "correct": "Huge"}), 4).await?;
    db.create_question(sub4, "CAUTIOUS means:", "multiple_choice", serde_json::json!({"choices": ["Reckless", "Careful", "Fast", "Happy"], "correct": "Careful"}), 5).await?;
    for i in 6..=10 {
        db.create_question(sub4, &format!("Vocabulary word {}", i), "multiple_choice", 
            serde_json::json!({"choices": ["A", "B", "C", "D"], "correct": "B"}), i.into()).await?;
    }
    
    // 5. Tool Matching - 5 min, form perception (sample 10 of 49)
    let sub5 = db.create_subtest(tool_id, "Tool Matching", 5, Some(300)).await?;
    for i in 1..=10 {
        db.create_question(sub5, "Which tool would you use to cut wood?", "multiple_choice", 
            serde_json::json!({"choices": ["Hammer", "Saw", "Screwdriver", "Wrench"], "correct": "Saw"}), i).await?;
    }
    
    // 6. Arithmetic Reasoning - 7 min, numerical reasoning (sample 10 of 25)
    let sub6 = db.create_subtest(tool_id, "Arithmetic Reasoning", 6, Some(420)).await?;
    db.create_question(sub6, "If 5 apples cost $2.50, how much do 8 apples cost?", "multiple_choice", 
        serde_json::json!({"choices": ["$3.50", "$4.00", "$4.50", "$5.00"], "correct": "$4.00"}), 1).await?;
    db.create_question(sub6, "A train travels 60 mph. How far in 2.5 hours?", "multiple_choice", 
        serde_json::json!({"choices": ["120 miles", "130 miles", "150 miles", "160 miles"], "correct": "150 miles"}), 2).await?;
    db.create_question(sub6, "If you buy 3 items at $4.50 each, what is the total?", "multiple_choice", 
        serde_json::json!({"choices": ["$12.50", "$13.00", "$13.50", "$14.00"], "correct": "$13.50"}), 3).await?;
    for i in 4..=10 {
        db.create_question(sub6, &format!("Word problem {}", i), "multiple_choice", 
            serde_json::json!({"choices": ["A", "B", "C", "D"], "correct": "C"}), i.into()).await?;
    }
    
    // 7. Form Matching - 5 min, perceptual speed (sample 10 of 60)
    let sub7 = db.create_subtest(tool_id, "Form Matching", 7, Some(300)).await?;
    for i in 1..=10 {
        db.create_question(sub7, "Which shape matches the target form?", "multiple_choice", 
            serde_json::json!({"choices": ["Form A", "Form B", "Form C", "Form D"], "correct": "Form B"}), i).await?;
    }
    
    // 8. Mark Making - motor speed (sample 10 of 60)
    let sub8 = db.create_subtest(tool_id, "Mark Making", 8, Some(300)).await?;
    for i in 1..=10 {
        db.create_question(sub8, "Make three marks in each square (speed test)", "multiple_choice", 
            serde_json::json!({"choices": ["Completed"], "correct": "Completed"}), i).await?;
    }

    Ok(())
}

async fn seed_epps(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("EPPS", "pair", "personality", "Edwards Personal Preference Schedule").await?;
    seed_epps_content(db, tool_id).await
}

async fn seed_epps_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // EPPS: 225 forced-choice pairs measuring 15 needs (simplified to 30 sample pairs)
    let subtest = db.create_subtest(tool_id, "EPPS Assessment", 1, Some(1800)).await?;
    
    // Sample pairs representing the 15 needs
    db.create_question(subtest, "Which statement describes you better?", "multiple_choice", 
        serde_json::json!({"choices": ["I like to accomplish difficult tasks", "I like to conform to rules"], "correct": ""}), 1).await?;
    db.create_question(subtest, "Which is more like you?", "multiple_choice", 
        serde_json::json!({"choices": ["I respect authority", "I enjoy organizing"], "correct": ""}), 2).await?;
    db.create_question(subtest, "Choose the statement that fits you best:", "multiple_choice", 
        serde_json::json!({"choices": ["I seek recognition", "I prefer being alone"], "correct": ""}), 3).await?;
    
    // Generate remaining sample pairs (simplified)
    for i in 4..=30 {
        db.create_question(subtest, "Which describes you better?", "multiple_choice", 
            serde_json::json!({"choices": ["Statement A", "Statement B"], "correct": ""}), i.into()).await?;
    }

    Ok(())
}

async fn seed_papi(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("PAPI", "pair", "personality", "PAPI Kostick").await?;
    seed_papi_content(db, tool_id).await
}

async fn seed_papi_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // PAPI: 90 forced-choice pairs measuring 20 dimensions (simplified to 30 sample pairs)
    let subtest = db.create_subtest(tool_id, "PAPI Assessment", 1, Some(1200)).await?;
    
    // Sample pairs
    db.create_question(subtest, "Which statement is more like you?", "multiple_choice", 
        serde_json::json!({"choices": ["I am a hard worker", "I am a leader"], "correct": ""}), 1).await?;
    db.create_question(subtest, "Choose one:", "multiple_choice", 
        serde_json::json!({"choices": ["I need to finish tasks", "I need to control others"], "correct": ""}), 2).await?;
    db.create_question(subtest, "Which fits you better?", "multiple_choice", 
        serde_json::json!({"choices": ["I follow rules strictly", "I work at my own pace"], "correct": ""}), 3).await?;
    
    for i in 4..=30 {
        db.create_question(subtest, "Which is more characteristic of you?", "multiple_choice", 
            serde_json::json!({"choices": ["Option A", "Option B"], "correct": ""}), i.into()).await?;
    }

    Ok(())
}

async fn seed_16pf(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("16PF", "choice", "personality", "Sixteen Personality Factor Questionnaire").await?;
    seed_16pf_content(db, tool_id).await
}

async fn seed_16pf_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // 16PF: 185 questions measuring 16 factors (simplified to 50 sample questions)
    let subtest = db.create_subtest(tool_id, "16PF Questionnaire", 1, Some(2400)).await?;
    
    // Sample questions for various factors
    db.create_question(subtest, "I am usually:", "multiple_choice", 
        serde_json::json!({"choices": ["Reserved", "Outgoing", "In between"], "correct": ""}), 1).await?;
    db.create_question(subtest, "I prefer:", "multiple_choice", 
        serde_json::json!({"choices": ["Concrete facts", "Abstract ideas", "Both equally"], "correct": ""}), 2).await?;
    db.create_question(subtest, "I tend to be:", "multiple_choice", 
        serde_json::json!({"choices": ["Emotional", "Stable", "Sometimes both"], "correct": ""}), 3).await?;
    db.create_question(subtest, "In groups, I am:", "multiple_choice", 
        serde_json::json!({"choices": ["Submissive", "Dominant", "Depends"], "correct": ""}), 4).await?;
    db.create_question(subtest, "I am:", "multiple_choice", 
        serde_json::json!({"choices": ["Serious", "Enthusiastic", "Varies"], "correct": ""}), 5).await?;
    
    for i in 6..=50 {
        db.create_question(subtest, "Rate yourself:", "multiple_choice", 
            serde_json::json!({"choices": ["Agree", "Disagree", "Uncertain"], "correct": ""}), i.into()).await?;
    }

    Ok(())
}

async fn seed_hexaco(db: &Database) -> Result<(), Error> {
    let tool_id = db.create_tool("HEXACO", "choice", "personality", "HEXACO / Big Five").await?;
    seed_hexaco_content(db, tool_id).await
}

async fn seed_hexaco_content(db: &Database, tool_id: i64) -> Result<(), Error> {
    // HEXACO: 100-200 items measuring 6 dimensions (simplified to 60 sample items)
    let subtest = db.create_subtest(tool_id, "HEXACO Inventory", 1, Some(1800)).await?;
    
    // Sample items for 6 HEXACO dimensions
    db.create_question(subtest, "I am honest even when it costs me:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 1).await?;
    db.create_question(subtest, "I feel emotional about others' problems:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 2).await?;
    db.create_question(subtest, "I am the life of the party:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 3).await?;
    db.create_question(subtest, "I forgive easily:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 4).await?;
    db.create_question(subtest, "I am always organized:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 5).await?;
    db.create_question(subtest, "I enjoy exploring new ideas:", "multiple_choice", 
        serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), 6).await?;
    
    for i in 7..=60 {
        let dimension = match i % 6 {
            1 => "I value honesty highly",
            2 => "I care about others' feelings",
            3 => "I enjoy social interactions",
            4 => "I am cooperative",
            5 => "I am detail-oriented",
            0 => "I am intellectually curious",
            _ => "Rate this statement"
        };
        db.create_question(subtest, dimension, "multiple_choice", 
            serde_json::json!({"choices": ["Strongly Disagree", "Disagree", "Neutral", "Agree", "Strongly Agree"], "correct": ""}), i.into()).await?;
    }

    Ok(())
}

pub async fn seed_dummy_results(db: &Database) -> Result<(), Error> {
    // 1. Create Dummy Candidates if they don't exist
    use crate::db::models;
    let candidates = ["alice", "bob", "charlie", "david", "eve"];
    let hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap_or_default();
    
    for (_i, name) in candidates.iter().enumerate() {
        // We use create_user but ignored error if exists. 
        // Ideally we should update the hash to ensure they can login.
        if let Err(_) = db.create_user(name, &hash, "participant").await {
            // User exists, update hash
            let _ = db.update_user_password(name, &hash).await;
        }
    }

    // 2. Create Dummy Events
    let event1 = db.create_event("Batch 1 - Alpha", Some("Initial hiring batch".to_string()), None).await?;
    let event2 = db.create_event("Batch 2 - Beta", Some("Secondary screening".to_string()), None).await?;

    // 3. Create Sessions & Reports (Completed Tests)
    // We'll create random results for different tools
    let tools = ["TIU", "KRAEPELIN", "DISC", "MBTI"];
    
    // Get user IDs
    let all_users = db.get_all_users().await?;
    let participants: Vec<&models::User> = all_users.iter().filter(|u| u.role == "participant").collect();

    for user in participants {
        for &tool_name in tools.iter() {
            // Randomly skip some tools for some users
            if (user.id + tool_name.len() as i64) % 3 == 0 { continue; }

            // Create Session
            let event_id = if user.id % 2 == 0 { event1 } else { event2 };
            // Participant ID is usually a code (e.g. K-123), but here we use username for simplicity or random string
            let participant_code = format!("{}-{}", user.username.to_uppercase(), 1000 + user.id);
            let session_id = db.create_session(event_id, &participant_code, None).await?;
            
            // Link session to user (update user_id)
            sqlx::query("UPDATE sessions SET user_id = ?, status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(user.id)
                .bind(session_id)
                .execute(&db.pool)
                .await?;

            // Create Report (Dummy Data)
            let score = ((user.id * 10) + (tool_name.len() as i64 * 5)) % 100;
            
            // For Kraepelin, we might want specific JSON structure in future, but for now flat score
            let report_json = serde_json::json!({
                "score": score,
                "raw_score": score * 2,
                "tool": tool_name,
                "details": {
                    "accuracy": 85 + (score % 15),
                    "speed": 60 + (score % 40)
                }
            });

            sqlx::query("INSERT INTO reports (session_id, scores, interpretations, generated_at) VALUES (?, ?, '{}', CURRENT_TIMESTAMP)")
                .bind(session_id)
                .bind(report_json)
                .execute(&db.pool)
                .await?;
        }
    }
    
    println!("Dummy results seeded.");
    Ok(())
}
