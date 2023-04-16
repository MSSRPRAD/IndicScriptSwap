// use crate::read_mappings::read_mappings;

// use crate::read_mappings::ScriptData;
mod convert;
mod data;
mod functions;
mod read_mappings;

fn main() {
    let foo = &data::HASH_MAP;

    let destination = foo.get("telugu").unwrap();
    let source = foo.get("slp1").unwrap();

    let input = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .
    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..
    yaM sarvaSElAH parikalpya vatsaM merO sTite dogDari dohadakze .
    BAsvanti ratnAni mahOzaDIS ca pfTUpadizwAM duduhur DaritrIm ..
    anantaratnapraBavasya yasya himaM na sOBAgyavilopi jAtam .
    eko hi dozo guRasaMnipAte nimajjatIndoH kiraRezv ivANkaH ..
    yaS cApsaroviBramamaRqanAnAM saMpAdayitrIM SiKarEr biBarti .
    balAhakacCedaviBaktarAgAm akAlasaMDyAm iva DAtumattAm ..
    AmeKalaM saMcaratAM GanAnAM cCAyAm aDaHsAnugatAM nizevya .
    udvejitA vfzwiBir ASrayante SfNgARi yasyAtapavanti sidDAH ..
    padaM tuzArasrutiDOtaraktaM yasminn adfzwvApi hatadvipAnAm .
    vidanti mArgaM naKaranDramuktEr muktAPalEH kesariRAM kirAtAH ..
    nyastAkzarA DAturasena yatra BUrjatvacaH kuYjarabinduSoRAH .
    vrajanti vidyADarasundarIRAm anaNgaleKakriyayopayogam ..
    yaH pUrayan kIcakaranDraBAgAn darImuKotTena samIraRena .
    udgAsyatAm icCati kiMnarARAM tAnapradAyitvam ivopagantum ..
    kapolakaRqUH kariBir vinetuM viGawwitAnAM saraladrumARAm .
    yatra srutakzIratayA prasUtaH sAnUni ganDaH suraBIkaroti ..
    vanecarARAM vanitAsaKAnAM darIgfhotsaNganizaktaBAsaH .
    Bavanti yatrOzaDayo rajanyAm atElapUrAH suratapradIpAH ..
    udvejayaty aNgulipArzRiBAgAn mArge SilIBUtahime 'pi yatra .
    na durvahaSroRipayoDarArtA Bindanti mandAM gatim aSvamuKyaH ..
    divAkarAd rakzati yo guhAsu lInaM divA BItam ivAnDakAram .
    kzudre 'pi nUnaM SaraRaM prapanne mamatvam uccEHSirasAM satIva ..
    lANgUlavikzepavisarpiSoBEr itas tataS candramarIcigOrEH .
    yasyArTayuktaM girirAjaSabdaM kurvanti vAlavyajanES camaryaH ..
    yatrAMSukAkzepavilajjitAnAM yadfcCayA kiMpuruzANganAnAm .
    darIgfhadvAravilambibimbAs tiraskariRyo jaladA Bavanti ..
    BAgIraTInirJarasIkarARAM voQA muhuH kampitadevadAruH .
    yad vAyur anvizwamfgEH kirAtEr Asevyate BinnaSiKaRqibarhaH ..
    saptarzihastAvacitAvaSezARy aDo vivasvAn parivartamAnaH .
    padmAni yasyAgrasaroruhARi praboDayaty UrDvamuKEr mayUKEH ..
    yajYANgayonitvam avekzya yasya sAraM DaritrIDaraRakzamaM ca .
    prajApatiH kalpitayajYaBAgaM SElADipatyaM svayam anvatizWat ..
    sa mAnasIM merusaKaH pitFRAM kanyAM kulasya sTitaye sTitijYaH .
    menAM munInAm api mAnanIyAm AtmAnurUpAM viDinopayeme ..
    kAlakrameRATa tayoH pravftte svarUpayogye surataprasaNge .
    manoramaM yOvanam udvahantyA garBo 'Bavad BUDararAjapatnyAH ..
    asUta sA nAgavaDUpaBogyaM mEnAkam amBoniDibadDasaKyam .
    krudDe 'pi pakzacCidi vftraSatrAv avedanAjYaM kuliSakzatAnAm ..
    aTAvamAnena pituH prayuktA dakzasya kanyA BavapUrvapatnI .
    satI satI yogavisfzwadehA tAM janmane SElavaDUM prapede ..
    sA BUDarARAm aDipena tasyAM samADimatyAm udapAdi BavyA .
    samyakprayogAd aparikzatAyAM nItAv ivotsAhaguRena saMpat ..
    prasannadik pAMsuviviktavAtaM SaNKasvanAnantarapuzpavfzwi .
    SarIriRAM sTAvarajaNgamAnAM suKAya tajjanmadinaM baBUva ..
    tayA duhitrA sutarAM savitrI sPuratpraBAmaRqalayA cakAse .
    vidUraBUmir navameGaSabdAd udBinnayA ratnaSalAkayeva ..
    dine dine sA parivarDamAnA labDodayA cAndramasIva leKA .
    pupoza lAvaRyamayAn viSezAY jyotsnAntarARIva kalAntarARi ..
    tAM pArvatIty ABijanena nAmnA banDupriyAM banDujano juhAva .
    u meti mAtrA tapaso nizidDA paScAd umAKyAM sumuKI jagAma ..
    mahIBftaH putravato 'pi dfzwis tasminn apatye na jagAma tfptim .
    anantapuzpasya maDor hi cUte dvirePamAlA saviSezasaNgA ..
    praBAmahatyA SiKayeva dIpas trimArgayeva tridivasya mArgaH .
    saMskAravatyeva girA manIzI tayA sa pUtaS ca viBUzitaS ca ..
    mandAkinIsEkatavedikABiH sA kandukEH kftrimaputrakES ca .
    reme muhur maDyagatA saKInAM krIqArasaM nirviSatIva bAlye ..
    tAM haMsamAlAH SaradIva gaNgAM mahOzaDiM naktam ivAtmaBAsaH .
    sTiropadeSAm upadeSakAle prapedire prAktanajanmavidyAH ..
    asaMBftaM maRqanam aNgayazwer anAsavAKyaM karaRaM madasya .
    kAmasya puzpavyatiriktam astraM bAlyAt paraM sATa vayaH prapede ..
    unmIlitaM tUlikayeva citraM sUryAMSuBir Binnam ivAravindam .
    baBUva tasyAS caturasraSoBi vapur viBaktaM navayOvanena ..
    aByunnatANguzWanaKapraBABir nikzepaRAd rAgam ivodgirantO .
    Ajahratus taccaraRO pfTivyAM sTalAravindaSriyam avyavasTAm ..
    sA rAjahaMsEr iva saMnatANgI gatezu lIlAYcitavikramezu .
    vyanIyata pratyupadeSalubDEr AditsuBir nUpurasiYjitAni ..
    vfttAnupUrve ca na cAtidIrGe jaNGe SuBe sfzwavatas tadIye .
    SezANganirmARaviDO viDAtur lAvaRya utpAdya ivAsa yatnaH ..
    nAgendrahastAs tvaci karkaSatvAd ekAntaSEtyAt kadalIviSezAH .
    labDvApi loke pariRAhi rUpaM jAtAs tadUrvor upamAnabAhyAH ..
    etAvatA nanv anumeyaSoBaM kAYcIguRasTAnam aninditAyAH .
    AropitaM yad giriSena paScAd ananyanArIkamanIyam aNkam ..
    tasyAH pravizwA natanABiranDraM rarAja tanvI navalomarAjiH .
    nIvIm atikramya sitetarasya tanmeKalAmaDyamaRer ivArciH ..
    maDyena sA vedivilagnamaDyA valitrayaM cAru baBAra bAlA .
    ArohaRArTaM navayOvanena kAmasya sopAnam iva prayuktam ..
    anyonyam utpIqayad utpalAkzyAH stanadvayaM pARqu taTA pravfdDam .
    maDye yaTA SyAmamuKasya tasya mfRAlasUtrAntaram apy alaByam ..
    SirIzapuzpADikasOkumAryO bAhU tadIyAv iti me vitarkaH .
    parAjitenApi kftO harasya yO kaRWapASO makaraDvajena ..
    kaRWasya tasyAH stanabanDurasya muktAkalApasya ca nistalasya .
    anyonyaSoBAjananAd baBUva sADAraRo BUzaRaBUzyaBAvaH ..
    candraM gatA padmaguRAn na BuNkte padmASritA cAndramasIm aBiKyAm .
    umAmuKaM tu pratipadya lolA dvisaMSrayAM prItim avApa lakzmIH ..
    puzpaM pravAlopahitaM yadi syAn muktAPalaM vA sPuwavidrumasTam .
    tato 'nukuryAd viSadasya tasyAs tAmrOzWaparyastarucaH smitasya ..
    svareRa tasyAm amftasruteva prajalpitAyAm aBijAtavAci .
    apy anyapuzwA pratikUlaSabdA Srotur vitantrIr iva tAqyamAnA ..
    pravAtanIlotpalanirviSezam aDIraviprekzitam AyatAkzyA .
    tayA gfhItaM nu mfgANganAByas tato gfhItaM nu mfgANganABiH ..
    tasyAH SalAkAYjananirmiteva kAntir Bruvor AnataleKayor yA .
    tAM vIkzya lIlAcaturAm anaNgaH svacApasOndaryamadaM mumoca ..
    lajjA tiraScAM yadi cetasi syAd asaMSayaM parvatarAjaputryAH .
    taM keSapASaM prasamIkzya kuryur vAlapriyatvaM SiTilaM camaryaH ..
    sarvopamAdravyasamuccayena yaTApradeSaM viniveSitena .
    sA nirmitA viSvasfjA prayatnAd ekasTasOndaryadidfkzayeva ..
    tAM nAradaH kAmacaraH kadA cit kanyAM kila prekzya pituH samIpe .
    samAdideSEkavaDUM BavitrIM premRA SarIrArDaharAM harasya ..
    guruH pragalBe 'pi vayasy ato 'syAs tasTO nivfttAnyavarABilAzaH .
    fte kfSAnor na hi mantrapUtam arhanti tejAMsy aparARi havyam ..
    ayAcitAraM na hi devadevam adriH sutAM grAhayituM SaSAka .
    aByarTanABaNgaBayena sADur mADyasTyam izwe 'py avalambate 'rTe ..
    yadEva pUrve janane SarIraM sA dakzarozAt sudatI sasarja .
    tadApraBfty eva vimuktasaNgaH patiH paSUnAm aparigraho 'BUt ..
    sa kfttivAsAs tapase yatAtmA gaNgApravAhokzitadevadAru .
    prasTaM himAdrer mfganABiganDi kiM cit kvaRatkiMnaram aDyuvAsa ..
    gaRA nameruprasavAvataMsA BUrjatvacaH sparSavatIr daDAnAH .
    manaHSilAvicCuritA nizeduH SEleyanadDezu SilAtalezu ..
    tuzArasaMGAtaSilAH KurAgrEH samulliKan darpakalaH kakudmAn .
    dfzwaH kaTaM cid gavayEr vivignEr asoQasiMhaDvanir unnanAda ..
    tatrAgnim ADAya samitsamidDaM svam eva mUrtyantaram azwamUrtiH .
    svayaM viDAtA tapasaH PalAnAm kenApi kAmena tapaS cacAra ..
    anarGyam arGyeRa tam adrinATaH svargOkasAm arcitam arcayitvA .
    ArADanAyAsya saKIsametAM samAdideSa prayatAM tanUjAm ..
    pratyarTiBUtAm api tAM samADeH SuSrUzamARAM giriSo 'numene .
    vikArahetO sati vikriyante yezAM na cetAMsi ta eva DIrAH ..
    avacitabalipuzpA vedisaMmArgadakzA niyamaviDijalAnAM barhizAM copanetrI .
    giriSam upacacAra pratyahaM sA sukeSI niyamitapariKedA tacCiraScandrapAdEH ..".to_string();
    let converted = convert::convert_roman_to_indic(&input, source, destination);
    println!("{}", converted);
}
